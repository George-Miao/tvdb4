// TVDB API V4
//
// Documentation of [TheTVDB](https://thetvdb.com/) API V4. All related information is linked from our [Github repo](https://github.com/thetvdb/v4-api). You might also want to use our [Postman collection] (https://www.getpostman.com/collections/7a9397ce69ff246f74d0) ## Authentication 1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call. 2. Executing this call will provide you with a bearer token, which is valid for 1 month. 3. Provide your bearer token for subsequent API calls by clicking Authorize below or including in the header of all direct API calls: `Authorization: Bearer [your-token]`  ## Notes 1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.
//
// The version of the OpenAPI document: 4.7.8
//
// Generated by: https://openapi-generator.tech

/// EpisodeExtendedRecord : extended episode record

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EpisodeExtendedRecord {
    #[serde(rename = "aired", skip_serializing_if = "Option::is_none")]
    pub aired: Option<String>,
    #[serde(rename = "airsAfterSeason", skip_serializing_if = "Option::is_none")]
    pub airs_after_season: Option<i32>,
    #[serde(rename = "airsBeforeEpisode", skip_serializing_if = "Option::is_none")]
    pub airs_before_episode: Option<i32>,
    #[serde(rename = "airsBeforeSeason", skip_serializing_if = "Option::is_none")]
    pub airs_before_season: Option<i32>,
    #[serde(rename = "awards", skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<crate::models::AwardBaseRecord>>,
    #[serde(rename = "characters", skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<crate::models::Character>>,
    #[serde(rename = "companies", skip_serializing_if = "Option::is_none")]
    pub companies: Option<Vec<crate::models::Company>>,
    #[serde(rename = "contentRatings", skip_serializing_if = "Option::is_none")]
    pub content_ratings: Option<Vec<crate::models::ContentRating>>,
    /// season, midseason, or series
    #[serde(rename = "finaleType", skip_serializing_if = "Option::is_none")]
    pub finale_type: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<i32>,
    #[serde(rename = "isMovie", skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<i64>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "linkedMovie", skip_serializing_if = "Option::is_none")]
    pub linked_movie: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameTranslations", skip_serializing_if = "Option::is_none")]
    pub name_translations: Option<Vec<String>>,
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::Company>>,
    #[serde(rename = "nominations", skip_serializing_if = "Option::is_none")]
    pub nominations: Option<Vec<crate::models::AwardNomineeBaseRecord>>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(
        rename = "overviewTranslations",
        skip_serializing_if = "Option::is_none"
    )]
    pub overview_translations: Option<Vec<String>>,
    #[serde(rename = "productionCode", skip_serializing_if = "Option::is_none")]
    pub production_code: Option<String>,
    #[serde(rename = "remoteIds", skip_serializing_if = "Option::is_none")]
    pub remote_ids: Option<Vec<crate::models::RemoteId>>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "seasonNumber", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<crate::models::SeasonBaseRecord>>,
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<i64>,
    #[serde(rename = "studios", skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<crate::models::Company>>,
    #[serde(rename = "tagOptions", skip_serializing_if = "Option::is_none")]
    pub tag_options: Option<Vec<crate::models::TagOption>>,
    #[serde(rename = "trailers", skip_serializing_if = "Option::is_none")]
    pub trailers: Option<Vec<crate::models::Trailer>>,
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<Box<crate::models::TranslationExtended>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
}

impl Default for EpisodeExtendedRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl EpisodeExtendedRecord {
    /// extended episode record
    pub fn new() -> EpisodeExtendedRecord {
        EpisodeExtendedRecord {
            aired: None,
            airs_after_season: None,
            airs_before_episode: None,
            airs_before_season: None,
            awards: None,
            characters: None,
            companies: None,
            content_ratings: None,
            finale_type: None,
            id: None,
            image: None,
            image_type: None,
            is_movie: None,
            last_updated: None,
            linked_movie: None,
            name: None,
            name_translations: None,
            networks: None,
            nominations: None,
            number: None,
            overview: None,
            overview_translations: None,
            production_code: None,
            remote_ids: None,
            runtime: None,
            season_number: None,
            seasons: None,
            series_id: None,
            studios: None,
            tag_options: None,
            trailers: None,
            translations: None,
            year: None,
        }
    }
}
