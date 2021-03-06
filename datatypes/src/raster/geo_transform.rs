use crate::primitives::Coordinate2D;
use serde::{Deserialize, Serialize};

/// This is a typedef for the `GDAL GeoTransform`. It represents an affine transformation matrix.
pub type GdalGeoTransform = [f64; 6];

/// The `GeoTransform` is a more user friendly representation of the `GDAL GeoTransform` affine transformation matrix.
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GeoTransform {
    pub upper_left_coordinate: Coordinate2D,
    pub x_pixel_size: f64,
    pub y_pixel_size: f64,
}

impl GeoTransform {
    /// Generates a new `GeoTransform`
    ///
    /// # Examples
    ///
    /// ```
    /// use geoengine_datatypes::raster::{GeoTransform};
    ///
    /// let geo_transform = GeoTransform::new((0.0, 0.0).into(), 1.0, -1.0);
    /// ```
    ///
    pub fn new(upper_left_coordinate: Coordinate2D, x_pixel_size: f64, y_pixel_size: f64) -> Self {
        Self {
            upper_left_coordinate,
            x_pixel_size,
            y_pixel_size,
        }
    }

    /// Generates a new `GeoTransform` with explicit x, y values of the upper left edge
    ///
    /// # Examples
    ///
    /// ```
    /// use geoengine_datatypes::raster::{GeoTransform};
    ///
    /// let geo_transform = GeoTransform::new_with_coordinate_x_y(0.0, 1.0, 0.0, -1.0);
    /// ```
    ///
    pub fn new_with_coordinate_x_y(
        upper_left_x_coordinate: f64,
        x_pixel_size: f64,
        upper_left_y_coordinate: f64,
        y_pixel_size: f64,
    ) -> Self {
        Self {
            upper_left_coordinate: (upper_left_x_coordinate, upper_left_y_coordinate).into(),
            x_pixel_size,
            y_pixel_size,
        }
    }

    /// Transforms a grid coordinate (row, column) ~ (y, x) into a SRS coordinate (x,y)
    /// See GDAL documentation for more details (including the two ignored parameters): <https://gdal.org/user/raster_data_model.html>
    ///
    /// # Examples
    ///
    /// ```
    /// use geoengine_datatypes::raster::{GeoTransform};
    /// use geoengine_datatypes::primitives::{Coordinate2D};
    ///
    /// let geo_transform = GeoTransform::new_with_coordinate_x_y(0.0, 1.0, 0.0, -1.0);
    /// assert_eq!(geo_transform.grid_2d_to_coordinate_2d((0, 0)), (0.0, 0.0).into())
    /// ```
    ///
    pub fn grid_2d_to_coordinate_2d(&self, grid_index: (usize, usize)) -> Coordinate2D {
        let (grid_index_y, grid_index_x) = grid_index;
        let coord_x = self.upper_left_coordinate.x + (grid_index_x as f64) * self.x_pixel_size;
        let coord_y = self.upper_left_coordinate.y + (grid_index_y as f64) * self.y_pixel_size;
        Coordinate2D::new(coord_x, coord_y)
    }

    /// Transforms an SRS coordinate (x,y) into a grid coordinate (row, column) ~ (y, x)
    ///
    /// # Examples
    ///
    /// ```
    /// use geoengine_datatypes::raster::{GeoTransform};
    /// use geoengine_datatypes::primitives::{Coordinate2D};
    ///
    /// let geo_transform = GeoTransform::new_with_coordinate_x_y(0.0, 1.0, 0.0, -1.0);
    /// assert_eq!(geo_transform.coordinate_2d_to_grid_2d((0.0, 0.0).into()), (0, 0))
    /// ```
    ///
    pub fn coordinate_2d_to_grid_2d(&self, coord: Coordinate2D) -> (usize, usize) {
        let grid_x_index = ((coord.x - self.upper_left_coordinate.x) / self.x_pixel_size) as usize;
        let grid_y_index = ((coord.y - self.upper_left_coordinate.y) / self.y_pixel_size) as usize;
        (grid_y_index, grid_x_index)
    }
}

impl Default for GeoTransform {
    fn default() -> Self {
        GeoTransform::new_with_coordinate_x_y(0.0, 1.0, 0.0, -1.0)
    }
}

impl From<GdalGeoTransform> for GeoTransform {
    fn from(gdal_geo_transform: GdalGeoTransform) -> Self {
        Self::new_with_coordinate_x_y(
            gdal_geo_transform[0],
            gdal_geo_transform[1],
            // gdal_geo_transform[2],
            gdal_geo_transform[3],
            // gdal_geo_transform[4],
            gdal_geo_transform[5],
        )
    }
}

impl Into<GdalGeoTransform> for GeoTransform {
    fn into(self) -> GdalGeoTransform {
        [
            self.upper_left_coordinate.x,
            self.x_pixel_size,
            0.0, // self.x_rotation,
            self.upper_left_coordinate.y,
            0.0, // self.y_rotation,
            self.y_pixel_size,
        ]
    }
}

#[cfg(test)]
mod tests {

    use crate::raster::GeoTransform;

    #[test]
    #[allow(clippy::float_cmp)]
    fn geo_transform_new() {
        let geo_transform = GeoTransform::new((0.0, 1.0).into(), 2.0, -3.0);
        assert_eq!(geo_transform.upper_left_coordinate.x, 0.0);
        assert_eq!(geo_transform.upper_left_coordinate.y, 1.0);
        assert_eq!(geo_transform.x_pixel_size, 2.0);
        assert_eq!(geo_transform.y_pixel_size, -3.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn geo_transform_new_with_coordinate_x_y() {
        let geo_transform = GeoTransform::new_with_coordinate_x_y(0.0, 1.0, 2.0, -3.0);
        assert_eq!(geo_transform.upper_left_coordinate.x, 0.0);
        assert_eq!(geo_transform.x_pixel_size, 1.0);
        assert_eq!(geo_transform.upper_left_coordinate.y, 2.0);
        assert_eq!(geo_transform.y_pixel_size, -3.0);
    }

    #[test]
    fn geo_transform_grid_2d_to_coordinate_2d() {
        let geo_transform = GeoTransform::new_with_coordinate_x_y(5.0, 1.0, 5.0, -1.0);
        assert_eq!(
            geo_transform.grid_2d_to_coordinate_2d((0, 0)),
            (5.0, 5.0).into()
        );
        assert_eq!(
            geo_transform.grid_2d_to_coordinate_2d((1, 1)),
            (6.0, 4.0).into()
        );
        assert_eq!(
            geo_transform.grid_2d_to_coordinate_2d((2, 2)),
            (7.0, 3.0).into()
        );
    }

    #[test]
    fn geo_transform_coordinate_2d_to_grid_2d() {
        let geo_transform = GeoTransform::new_with_coordinate_x_y(5.0, 1.0, 5.0, -1.0);
        assert_eq!(
            geo_transform.coordinate_2d_to_grid_2d((5.0, 5.0).into()),
            (0, 0)
        );
        assert_eq!(
            geo_transform.coordinate_2d_to_grid_2d((6.0, 4.0).into()),
            (1, 1)
        );
        assert_eq!(
            geo_transform.coordinate_2d_to_grid_2d((7.0, 3.0).into()),
            (2, 2)
        );
    }
}
