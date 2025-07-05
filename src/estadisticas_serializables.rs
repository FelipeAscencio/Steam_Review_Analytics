//! Este módulo contiene la lógica de las estadísticas finales y serializadas del programa.

// Imports de crates externas.
use serde::Serialize;
use std::collections::HashMap;

// Imports de funciones/estructuras propias.
use crate::estadisticas::EstadisticasGlobales;

// Constantes.
const MAX_TOP_JUEGOS: usize = 3;
const MAX_TOP_REVIEWS_JUEGOS: usize = 3;
const MAX_TOP_IDIOMAS: usize = 3;
const MAX_TOP_REVIEWS_IDIOMAS: usize = 10;

// Mensajes.
const ERROR_INFORMACION_IDIOMAS: &str = "La información de idiomas debería estar presente";

// Estructura usada para serializar la información de los juegos.
/// Estructura que representa la información serializable de un juego, incluyendo
/// el número total de reviews, las reviews por idioma y las mejores reviews.
#[derive(Debug, Serialize)]
pub struct InfoJuegoSerializable {
    pub cantidad_total: usize,
    pub por_idioma: HashMap<String, usize>,
    pub mejores_reviews: HashMap<String, MejorReview>,
}

// Estructura usada para serializar la información de los idiomas.
/// Estructura que representa la información serializable de un idioma, incluyendo
/// el número total de reviews y el top de mejores reviews para ese idioma.
#[derive(Debug, Serialize)]
pub struct InfoIdiomaSerializable {
    pub cantidad_total: usize,
    pub top_reviews: Vec<MejorReview>,
}

// Estructura usada para serializar las reviews.
/// Estructura que representa una review, con su texto y el número de votos que ha recibido.
#[derive(Debug, Serialize, Clone)]
pub struct MejorReview {
    pub texto: String,
    pub votos: u32,
}

// Estructura usada para formatear todo el resultado obtenido al ".json" final.
/// Estructura que representa las estadísticas globales serializadas, incluyendo
/// los juegos, idiomas y el top de reviews por idioma.
#[derive(Debug, Serialize)]
pub struct EstadisticasGlobalesSerializable {
    pub juegos: HashMap<String, InfoJuegoSerializable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idiomas: Option<HashMap<String, InfoIdiomaSerializable>>,
    pub top_idiomas: HashMap<String, Vec<MejorReview>>,
}

// Estructura para formatear el esquema general del ".json" con padrón incluido.
/// Estructura que representa la salida final del programa, con el padrón y los
/// juegos y idiomas más relevantes.
#[derive(Serialize)]
pub struct SalidaFinal {
    pub padron: u32,
    pub top_games: Vec<TopGame>,
    pub top_languages: Vec<TopLanguage>,
}

// Estructura usada para mostrar en el ".json" los juegos con más reviews.
/// Estructura que representa un juego en el top, con el nombre del juego, la cantidad
/// de reviews y los idiomas con las mejores reviews.
#[derive(Serialize, PartialEq)]
pub struct TopGame {
    pub game: String,
    pub review_count: u32,
    pub languages: Vec<IdiomaPorJuego>,
}

// Estructura auxiliar de 'TopGame'.
/// Estructura que representa un idioma de un juego en el top, con el idioma, la cantidad
/// de reviews y la mejor review del idioma.
#[derive(Serialize, PartialEq)]
pub struct IdiomaPorJuego {
    pub language: String,
    pub review_count: u32,
    pub top_review: String,
    pub top_review_votes: u32,
}

// Estructura usada para mostrar en el ".json" los idiomas con más reviews.
/// Estructura que representa un idioma en el top, con el nombre del idioma, la cantidad
/// de reviews y las mejores reviews para ese idioma.
#[derive(Serialize)]
pub struct TopLanguage {
    pub language: String,
    pub review_count: u32,
    pub top_reviews: Vec<ReviewIdioma>,
}

// Estructura auxiliar de 'TopLanguage'.
/// Estructura que representa una review de un idioma en el top, con el texto de la review
/// y el número de votos.
#[derive(Serialize)]
pub struct ReviewIdioma {
    pub review: String,
    pub votes: u32,
}

// Función que hace el filtrado final y la serialización para luego escribir el ".json".
///
/// # Parámetros
/// - `est`: Estadísticas globales que se utilizarán para filtrar y serializar los datos.
///
/// # Retorna
/// - Devuelve un objeto de tipo `EstadisticasGlobalesSerializable` con los datos filtrados y serializados.
pub fn filtrar_top3(est: &EstadisticasGlobales) -> EstadisticasGlobalesSerializable {
    let juegos = filtrar_top_juegos_con_reviews(est);
    let idiomas_serializables = filtrar_idiomas_serializables(est);
    let top_idiomas = obtener_top_reviews_por_idioma(&idiomas_serializables);

    EstadisticasGlobalesSerializable {
        juegos,
        idiomas: Some(idiomas_serializables),
        top_idiomas,
    }
}

// Función que filtra los juegos con más cantidad de reviews.
///
/// # Parámetros
/// - `est`: Estadísticas globales que contienen la información de los juegos.
///
/// # Retorna
/// - Devuelve un `HashMap` con los juegos filtrados y serializados.
fn filtrar_top_juegos_con_reviews(
    est: &EstadisticasGlobales,
) -> HashMap<String, InfoJuegoSerializable> {
    let mut juegos_vec: Vec<_> = est.juegos.iter().collect();
    juegos_vec.sort_by_key(|(_, info)| std::cmp::Reverse(info.cantidad_total));
    juegos_vec.truncate(MAX_TOP_JUEGOS);
    juegos_vec
        .into_iter()
        .map(|(juego, info)| {
            let mut idiomas_vec: Vec<_> = info.por_idioma.iter().collect();
            idiomas_vec
                .sort_by_key(|(idioma, count)| (std::cmp::Reverse(**count), (*idioma).clone()));
            idiomas_vec.truncate(MAX_TOP_REVIEWS_JUEGOS);
            let por_idioma = idiomas_vec
                .iter()
                .map(|(idioma, count)| ((*idioma).clone(), **count))
                .collect();

            let mejores_reviews = idiomas_vec
                .iter()
                .filter_map(|(idioma, _)| {
                    info.mejores_reviews.get(*idioma).map(|(texto, votos)| {
                        (
                            (*idioma).clone(),
                            MejorReview {
                                texto: texto.clone(),
                                votos: *votos,
                            },
                        )
                    })
                })
                .collect();

            (
                juego.clone(),
                InfoJuegoSerializable {
                    cantidad_total: info.cantidad_total,
                    por_idioma,
                    mejores_reviews,
                },
            )
        })
        .collect()
}

// Función que filtra los idiomas con más cantidad de reviews.
///
/// # Parámetros
/// - `est`: Estadísticas globales que contienen la información de los idiomas.
///
/// # Retorna
/// - Devuelve un `HashMap` con los idiomas filtrados y serializados.
fn filtrar_idiomas_serializables(
    est: &EstadisticasGlobales,
) -> HashMap<String, InfoIdiomaSerializable> {
    let mut idiomas_vec: Vec<_> = est.por_idioma.iter().collect();
    idiomas_vec.sort_by_key(|(_, info)| std::cmp::Reverse(info.cantidad_total));
    idiomas_vec.truncate(MAX_TOP_IDIOMAS);
    idiomas_vec
        .into_iter()
        .map(|(idioma, info)| {
            let top_reviews = info
                .top_reviews
                .iter()
                .take(MAX_TOP_REVIEWS_IDIOMAS)
                .map(|(texto, votos)| MejorReview {
                    texto: texto.clone(),
                    votos: *votos,
                })
                .collect();

            (
                idioma.clone(),
                InfoIdiomaSerializable {
                    cantidad_total: info.cantidad_total,
                    top_reviews,
                },
            )
        })
        .collect()
}

// Función que filtra las reviews con más votos positivos por idioma.
///
/// # Parámetros
/// - `idiomas`: Un `HashMap` que contiene la información de los idiomas.
///
/// # Retorna
/// - Devuelve un `HashMap` con los idiomas y sus top reviews.
fn obtener_top_reviews_por_idioma(
    idiomas: &HashMap<String, InfoIdiomaSerializable>,
) -> HashMap<String, Vec<MejorReview>> {
    let mut idiomas_vec = idiomas.iter().collect::<Vec<_>>();
    idiomas_vec.sort_by_key(|(_, info)| std::cmp::Reverse(info.cantidad_total));
    idiomas_vec
        .into_iter()
        .map(|(idioma, info)| (idioma.clone(), info.top_reviews.clone()))
        .collect()
}

// Trait que se encarga de crear la salida final para el ".json".
///
/// # Método
/// - `a_salida_final(padron: u32)`
///     - Convierte las estadísticas globales serializadas en una salida final.
///
pub trait ASalidaFinal {
    fn a_salida_final(&self, padron: u32) -> SalidaFinal;
}

// Método que implementa la lógica para crear la salida final.
impl ASalidaFinal for EstadisticasGlobalesSerializable {
    fn a_salida_final(&self, padron: u32) -> SalidaFinal {
        let top_games = convertir_top_games(&self.juegos);
        let top_languages =
            convertir_top_languages(self.idiomas.as_ref().expect(ERROR_INFORMACION_IDIOMAS));

        SalidaFinal {
            padron,
            top_games,
            top_languages,
        }
    }
}

// Función que implementa la lógica de conversión de los juegos para la salida.
///
/// # Parámetros
/// - `juegos`: Un `HashMap` con la información de los juegos.
///
/// # Retorna
/// - Devuelve un `Vec<TopGame>` con la información de los juegos en formato adecuado.
fn convertir_top_games(juegos: &HashMap<String, InfoJuegoSerializable>) -> Vec<TopGame> {
    let mut top_games: Vec<TopGame> = juegos
        .iter()
        .map(|(juego, info)| {
            let mut languages: Vec<IdiomaPorJuego> = info
                .por_idioma
                .iter()
                .map(|(idioma, count)| {
                    let (texto, votos) = info
                        .mejores_reviews
                        .get(idioma)
                        .map(|mr| (mr.texto.clone(), mr.votos))
                        .unwrap_or_else(|| ("".to_string(), 0));

                    IdiomaPorJuego {
                        language: idioma.clone(),
                        review_count: *count as u32,
                        top_review: texto,
                        top_review_votes: votos,
                    }
                })
                .collect();

            languages.sort_by(|a, b| b.review_count.cmp(&a.review_count));
            TopGame {
                game: juego.clone(),
                review_count: info.cantidad_total as u32,
                languages,
            }
        })
        .collect();

    top_games.sort_by(|a, b| b.review_count.cmp(&a.review_count));
    top_games
}

// Función que implementa la lógica de conversión de los idiomas para la salida.
///
/// # Parámetros
/// - `idiomas`: Un `HashMap` con la información de los idiomas.
///
/// # Retorna
/// - Devuelve un `Vec<TopLanguage>` con la información de los idiomas en formato adecuado.
fn convertir_top_languages(idiomas: &HashMap<String, InfoIdiomaSerializable>) -> Vec<TopLanguage> {
    let mut top_languages: Vec<TopLanguage> = idiomas
        .iter()
        .map(|(idioma, info)| TopLanguage {
            language: idioma.clone(),
            review_count: info.cantidad_total as u32,
            top_reviews: info
                .top_reviews
                .iter()
                .map(|mr| ReviewIdioma {
                    review: mr.texto.clone(),
                    votes: mr.votos,
                })
                .collect(),
        })
        .collect();

    top_languages.sort_by(|a, b| b.review_count.cmp(&a.review_count));
    top_languages
}
