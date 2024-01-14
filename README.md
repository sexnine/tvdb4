# Rust client for TVDB API v4

This API is generated from the [official OpenAPI spec](https://thetvdb.github.io/v4-api) using [OpenAPI Generator](https://openapi-generator.tech). However, there's some [known errors](https://github.com/thetvdb/v4-api/issues/314) with the data returned, so use at your own risk and this lib is not affiliated with thetvdb.com in any way

## Notices from the TVDB

### Authentication

1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call.
2. Executing this call will provide you with a bearer token, which is valid for 1 month.
3. Provide your bearer token for subsequent API calls by including it in headers.

### Notes

1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.

## Overview

- API version: 4.7.8
- Package version: 4.7.8

## Installation

Add following line to `Cargo.toml` of your project:

```toml
[dependencies]
tvdb4 = "4.7.8"
```

## Documentation for API Endpoints

All URIs are relative to *<https://api4.thetvdb.com/v4>*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ArtworkApi* | [**get_artwork_base**](docs/ArtworkApi.md#get_artwork_base) | **GET** /artwork/{id} |
*ArtworkApi* | [**get_artwork_extended**](docs/ArtworkApi.md#get_artwork_extended) | **GET** /artwork/{id}/extended |
*ArtworkStatusesApi* | [**get_all_artwork_statuses**](docs/ArtworkStatusesApi.md#get_all_artwork_statuses) | **GET** /artwork/statuses |
*ArtworkTypesApi* | [**get_all_artwork_types**](docs/ArtworkTypesApi.md#get_all_artwork_types) | **GET** /artwork/types |
*AwardCategoriesApi* | [**get_award_category**](docs/AwardCategoriesApi.md#get_award_category) | **GET** /awards/categories/{id} |
*AwardCategoriesApi* | [**get_award_category_extended**](docs/AwardCategoriesApi.md#get_award_category_extended) | **GET** /awards/categories/{id}/extended |
*AwardsApi* | [**get_all_awards**](docs/AwardsApi.md#get_all_awards) | **GET** /awards |
*AwardsApi* | [**get_award**](docs/AwardsApi.md#get_award) | **GET** /awards/{id} |
*AwardsApi* | [**get_award_extended**](docs/AwardsApi.md#get_award_extended) | **GET** /awards/{id}/extended |
*CharactersApi* | [**get_character_base**](docs/CharactersApi.md#get_character_base) | **GET** /characters/{id} |
*CompaniesApi* | [**get_all_companies**](docs/CompaniesApi.md#get_all_companies) | **GET** /companies |
*CompaniesApi* | [**get_company**](docs/CompaniesApi.md#get_company) | **GET** /companies/{id} |
*CompaniesApi* | [**get_company_types**](docs/CompaniesApi.md#get_company_types) | **GET** /companies/types |
*ContentRatingsApi* | [**get_all_content_ratings**](docs/ContentRatingsApi.md#get_all_content_ratings) | **GET** /content/ratings |
*CountriesApi* | [**get_all_countries**](docs/CountriesApi.md#get_all_countries) | **GET** /countries |
*EntityTypesApi* | [**get_entity_types**](docs/EntityTypesApi.md#get_entity_types) | **GET** /entities |
*EpisodesApi* | [**get_all_episodes**](docs/EpisodesApi.md#get_all_episodes) | **GET** /episodes |
*EpisodesApi* | [**get_episode_base**](docs/EpisodesApi.md#get_episode_base) | **GET** /episodes/{id} |
*EpisodesApi* | [**get_episode_extended**](docs/EpisodesApi.md#get_episode_extended) | **GET** /episodes/{id}/extended |
*EpisodesApi* | [**get_episode_translation**](docs/EpisodesApi.md#get_episode_translation) | **GET** /episodes/{id}/translations/{language} |
*FavoritesApi* | [**create_user_favorites**](docs/FavoritesApi.md#create_user_favorites) | **POST** /user/favorites |
*FavoritesApi* | [**get_user_favorites**](docs/FavoritesApi.md#get_user_favorites) | **GET** /user/favorites |
*GendersApi* | [**get_all_genders**](docs/GendersApi.md#get_all_genders) | **GET** /genders |
*GenresApi* | [**get_all_genres**](docs/GenresApi.md#get_all_genres) | **GET** /genres |
*GenresApi* | [**get_genre_base**](docs/GenresApi.md#get_genre_base) | **GET** /genres/{id} |
*InspirationTypesApi* | [**get_all_inspiration_types**](docs/InspirationTypesApi.md#get_all_inspiration_types) | **GET** /inspiration/types |
*LanguagesApi* | [**get_all_languages**](docs/LanguagesApi.md#get_all_languages) | **GET** /languages |
*ListsApi* | [**get_all_lists**](docs/ListsApi.md#get_all_lists) | **GET** /lists |
*ListsApi* | [**get_list**](docs/ListsApi.md#get_list) | **GET** /lists/{id} |
*ListsApi* | [**get_list_by_slug**](docs/ListsApi.md#get_list_by_slug) | **GET** /lists/slug/{slug} |
*ListsApi* | [**get_list_extended**](docs/ListsApi.md#get_list_extended) | **GET** /lists/{id}/extended |
*ListsApi* | [**get_list_translation**](docs/ListsApi.md#get_list_translation) | **GET** /lists/{id}/translations/{language} |
*LoginApi* | [**login_post**](docs/LoginApi.md#login_post) | **POST** /login | create an auth token. The token has one month validation length.
*MovieStatusesApi* | [**get_all_movie_statuses**](docs/MovieStatusesApi.md#get_all_movie_statuses) | **GET** /movies/statuses |
*MoviesApi* | [**get_all_movie**](docs/MoviesApi.md#get_all_movie) | **GET** /movies |
*MoviesApi* | [**get_movie_base**](docs/MoviesApi.md#get_movie_base) | **GET** /movies/{id} |
*MoviesApi* | [**get_movie_base_by_slug**](docs/MoviesApi.md#get_movie_base_by_slug) | **GET** /movies/slug/{slug} |
*MoviesApi* | [**get_movie_extended**](docs/MoviesApi.md#get_movie_extended) | **GET** /movies/{id}/extended |
*MoviesApi* | [**get_movie_translation**](docs/MoviesApi.md#get_movie_translation) | **GET** /movies/{id}/translations/{language} |
*MoviesApi* | [**get_movies_filter**](docs/MoviesApi.md#get_movies_filter) | **GET** /movies/filter |
*PeopleApi* | [**get_all_people**](docs/PeopleApi.md#get_all_people) | **GET** /people |
*PeopleApi* | [**get_people_base**](docs/PeopleApi.md#get_people_base) | **GET** /people/{id} |
*PeopleApi* | [**get_people_extended**](docs/PeopleApi.md#get_people_extended) | **GET** /people/{id}/extended |
*PeopleApi* | [**get_people_translation**](docs/PeopleApi.md#get_people_translation) | **GET** /people/{id}/translations/{language} |
*PeopleTypesApi* | [**get_all_people_types**](docs/PeopleTypesApi.md#get_all_people_types) | **GET** /people/types |
*SearchApi* | [**get_search_results**](docs/SearchApi.md#get_search_results) | **GET** /search |
*SearchApi* | [**get_search_results_by_remote_id**](docs/SearchApi.md#get_search_results_by_remote_id) | **GET** /search/remoteid/{remoteId} |
*SeasonsApi* | [**get_all_seasons**](docs/SeasonsApi.md#get_all_seasons) | **GET** /seasons |
*SeasonsApi* | [**get_season_base**](docs/SeasonsApi.md#get_season_base) | **GET** /seasons/{id} |
*SeasonsApi* | [**get_season_extended**](docs/SeasonsApi.md#get_season_extended) | **GET** /seasons/{id}/extended |
*SeasonsApi* | [**get_season_translation**](docs/SeasonsApi.md#get_season_translation) | **GET** /seasons/{id}/translations/{language} |
*SeasonsApi* | [**get_season_types**](docs/SeasonsApi.md#get_season_types) | **GET** /seasons/types |
*SeriesApi* | [**get_all_series**](docs/SeriesApi.md#get_all_series) | **GET** /series |
*SeriesApi* | [**get_series_artworks**](docs/SeriesApi.md#get_series_artworks) | **GET** /series/{id}/artworks |
*SeriesApi* | [**get_series_base**](docs/SeriesApi.md#get_series_base) | **GET** /series/{id} |
*SeriesApi* | [**get_series_base_by_slug**](docs/SeriesApi.md#get_series_base_by_slug) | **GET** /series/slug/{slug} |
*SeriesApi* | [**get_series_episodes**](docs/SeriesApi.md#get_series_episodes) | **GET** /series/{id}/episodes/{season-type} |
*SeriesApi* | [**get_series_extended**](docs/SeriesApi.md#get_series_extended) | **GET** /series/{id}/extended |
*SeriesApi* | [**get_series_filter**](docs/SeriesApi.md#get_series_filter) | **GET** /series/filter |
*SeriesApi* | [**get_series_next_aired**](docs/SeriesApi.md#get_series_next_aired) | **GET** /series/{id}/nextAired |
*SeriesApi* | [**get_series_season_episodes_translated**](docs/SeriesApi.md#get_series_season_episodes_translated) | **GET** /series/{id}/episodes/{season-type}/{lang} |
*SeriesApi* | [**get_series_translation**](docs/SeriesApi.md#get_series_translation) | **GET** /series/{id}/translations/{language} |
*SeriesStatusesApi* | [**get_all_series_statuses**](docs/SeriesStatusesApi.md#get_all_series_statuses) | **GET** /series/statuses |
*SourceTypesApi* | [**get_all_source_types**](docs/SourceTypesApi.md#get_all_source_types) | **GET** /sources/types |
*UpdatesApi* | [**updates**](docs/UpdatesApi.md#updates) | **GET** /updates |
*UserInfoApi* | [**get_user_info**](docs/UserInfoApi.md#get_user_info) | **GET** /user |
*UserInfoApi* | [**get_user_info_by_id**](docs/UserInfoApi.md#get_user_info_by_id) | **GET** /user/{id} |

## Documentation For Models

- [Alias](docs/Alias.md)
- [ArtworkBaseRecord](docs/ArtworkBaseRecord.md)
- [ArtworkExtendedRecord](docs/ArtworkExtendedRecord.md)
- [ArtworkStatus](docs/ArtworkStatus.md)
- [ArtworkType](docs/ArtworkType.md)
- [AwardBaseRecord](docs/AwardBaseRecord.md)
- [AwardCategoryBaseRecord](docs/AwardCategoryBaseRecord.md)
- [AwardCategoryExtendedRecord](docs/AwardCategoryExtendedRecord.md)
- [AwardExtendedRecord](docs/AwardExtendedRecord.md)
- [AwardNomineeBaseRecord](docs/AwardNomineeBaseRecord.md)
- [Biography](docs/Biography.md)
- [Character](docs/Character.md)
- [Companies](docs/Companies.md)
- [Company](docs/Company.md)
- [CompanyRelationShip](docs/CompanyRelationShip.md)
- [CompanyType](docs/CompanyType.md)
- [ContentRating](docs/ContentRating.md)
- [Country](docs/Country.md)
- [Entity](docs/Entity.md)
- [EntityType](docs/EntityType.md)
- [EntityUpdate](docs/EntityUpdate.md)
- [EpisodeBaseRecord](docs/EpisodeBaseRecord.md)
- [EpisodeExtendedRecord](docs/EpisodeExtendedRecord.md)
- [FavoriteRecord](docs/FavoriteRecord.md)
- [Favorites](docs/Favorites.md)
- [Gender](docs/Gender.md)
- [GenreBaseRecord](docs/GenreBaseRecord.md)
- [GetAllArtworkStatuses200Response](docs/GetAllArtworkStatuses200Response.md)
- [GetAllArtworkTypes200Response](docs/GetAllArtworkTypes200Response.md)
- [GetAllAwards200Response](docs/GetAllAwards200Response.md)
- [GetAllCompanies200Response](docs/GetAllCompanies200Response.md)
- [GetAllContentRatings200Response](docs/GetAllContentRatings200Response.md)
- [GetAllCountries200Response](docs/GetAllCountries200Response.md)
- [GetAllEpisodes200Response](docs/GetAllEpisodes200Response.md)
- [GetAllGenders200Response](docs/GetAllGenders200Response.md)
- [GetAllGenres200Response](docs/GetAllGenres200Response.md)
- [GetAllInspirationTypes200Response](docs/GetAllInspirationTypes200Response.md)
- [GetAllLanguages200Response](docs/GetAllLanguages200Response.md)
- [GetAllLists200Response](docs/GetAllLists200Response.md)
- [GetAllMovie200Response](docs/GetAllMovie200Response.md)
- [GetAllMovieStatuses200Response](docs/GetAllMovieStatuses200Response.md)
- [GetAllPeople200Response](docs/GetAllPeople200Response.md)
- [GetAllPeopleTypes200Response](docs/GetAllPeopleTypes200Response.md)
- [GetAllSeasons200Response](docs/GetAllSeasons200Response.md)
- [GetAllSeries200Response](docs/GetAllSeries200Response.md)
- [GetAllSourceTypes200Response](docs/GetAllSourceTypes200Response.md)
- [GetArtworkBase200Response](docs/GetArtworkBase200Response.md)
- [GetArtworkExtended200Response](docs/GetArtworkExtended200Response.md)
- [GetAward200Response](docs/GetAward200Response.md)
- [GetAwardCategory200Response](docs/GetAwardCategory200Response.md)
- [GetAwardCategoryExtended200Response](docs/GetAwardCategoryExtended200Response.md)
- [GetAwardExtended200Response](docs/GetAwardExtended200Response.md)
- [GetCharacterBase200Response](docs/GetCharacterBase200Response.md)
- [GetCompany200Response](docs/GetCompany200Response.md)
- [GetCompanyTypes200Response](docs/GetCompanyTypes200Response.md)
- [GetEntityTypes200Response](docs/GetEntityTypes200Response.md)
- [GetEpisodeBase200Response](docs/GetEpisodeBase200Response.md)
- [GetEpisodeExtended200Response](docs/GetEpisodeExtended200Response.md)
- [GetEpisodeTranslation200Response](docs/GetEpisodeTranslation200Response.md)
- [GetGenreBase200Response](docs/GetGenreBase200Response.md)
- [GetList200Response](docs/GetList200Response.md)
- [GetListExtended200Response](docs/GetListExtended200Response.md)
- [GetListTranslation200Response](docs/GetListTranslation200Response.md)
- [GetMovieBase200Response](docs/GetMovieBase200Response.md)
- [GetMovieExtended200Response](docs/GetMovieExtended200Response.md)
- [GetMoviesFilter200Response](docs/GetMoviesFilter200Response.md)
- [GetPeopleBase200Response](docs/GetPeopleBase200Response.md)
- [GetPeopleExtended200Response](docs/GetPeopleExtended200Response.md)
- [GetSearchResults200Response](docs/GetSearchResults200Response.md)
- [GetSearchResultsByRemoteId200Response](docs/GetSearchResultsByRemoteId200Response.md)
- [GetSeasonBase200Response](docs/GetSeasonBase200Response.md)
- [GetSeasonExtended200Response](docs/GetSeasonExtended200Response.md)
- [GetSeasonTypes200Response](docs/GetSeasonTypes200Response.md)
- [GetSeriesArtworks200Response](docs/GetSeriesArtworks200Response.md)
- [GetSeriesBase200Response](docs/GetSeriesBase200Response.md)
- [GetSeriesEpisodes200Response](docs/GetSeriesEpisodes200Response.md)
- [GetSeriesEpisodes200ResponseData](docs/GetSeriesEpisodes200ResponseData.md)
- [GetSeriesFilter200Response](docs/GetSeriesFilter200Response.md)
- [GetSeriesSeasonEpisodesTranslated200Response](docs/GetSeriesSeasonEpisodesTranslated200Response.md)
- [GetSeriesSeasonEpisodesTranslated200ResponseData](docs/GetSeriesSeasonEpisodesTranslated200ResponseData.md)
- [GetUserFavorites200Response](docs/GetUserFavorites200Response.md)
- [GetUserInfo200Response](docs/GetUserInfo200Response.md)
- [Inspiration](docs/Inspiration.md)
- [InspirationType](docs/InspirationType.md)
- [Language](docs/Language.md)
- [Links](docs/Links.md)
- [ListBaseRecord](docs/ListBaseRecord.md)
- [ListExtendedRecord](docs/ListExtendedRecord.md)
- [LoginPost200Response](docs/LoginPost200Response.md)
- [LoginPost200ResponseData](docs/LoginPost200ResponseData.md)
- [LoginPostRequest](docs/LoginPostRequest.md)
- [MovieBaseRecord](docs/MovieBaseRecord.md)
- [MovieExtendedRecord](docs/MovieExtendedRecord.md)
- [ParentCompany](docs/ParentCompany.md)
- [PeopleBaseRecord](docs/PeopleBaseRecord.md)
- [PeopleExtendedRecord](docs/PeopleExtendedRecord.md)
- [PeopleType](docs/PeopleType.md)
- [ProductionCountry](docs/ProductionCountry.md)
- [RecordInfo](docs/RecordInfo.md)
- [Release](docs/Release.md)
- [RemoteId](docs/RemoteId.md)
- [SearchByRemoteIdResult](docs/SearchByRemoteIdResult.md)
- [SearchResult](docs/SearchResult.md)
- [SeasonBaseRecord](docs/SeasonBaseRecord.md)
- [SeasonExtendedRecord](docs/SeasonExtendedRecord.md)
- [SeasonType](docs/SeasonType.md)
- [SeriesAirsDays](docs/SeriesAirsDays.md)
- [SeriesBaseRecord](docs/SeriesBaseRecord.md)
- [SeriesExtendedRecord](docs/SeriesExtendedRecord.md)
- [SourceType](docs/SourceType.md)
- [Status](docs/Status.md)
- [StudioBaseRecord](docs/StudioBaseRecord.md)
- [Tag](docs/Tag.md)
- [TagOption](docs/TagOption.md)
- [TagOptionEntity](docs/TagOptionEntity.md)
- [Trailer](docs/Trailer.md)
- [Translation](docs/Translation.md)
- [TranslationExtended](docs/TranslationExtended.md)
- [TranslationSimple](docs/TranslationSimple.md)
- [Updates200Response](docs/Updates200Response.md)
- [UserInfo](docs/UserInfo.md)
