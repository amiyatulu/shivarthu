use crate::*;

impl<T: Config> Pallet<T> {
	pub(super) fn mean_integer(data: &Vec<i64>) -> Option<i64> {
		let data_mul_sum = data.iter().sum::<i64>();
		let count = data.len();

		match count {
			positive if positive > 0 => Some(data_mul_sum / count as i64),
			_ => None,
		}
	}

	pub(super) fn std_deviation_interger(data: &Vec<i64>) -> Option<i64> {
		match (Self::mean_integer(data), data.len()) {
			(Some(data_mean), count) if count > 0 => {
				let variance = data
					.iter()
					.map(|value| {
						let diff = data_mean.checked_sub(*value as i64).unwrap();
						diff * diff
					})
					.sum::<i64>() / count as i64;

				Some(variance.sqrt())
			},
			_ => None,
		}
	}

	pub(super) fn calculate_new_mean(
		data: &Vec<i64>,
		mean: Option<i64>,
		sd: Option<i64>,
	) -> Option<i64> {
		let mut new_items = vec![];
		for x in data {
			if *x >= mean.unwrap().checked_sub(sd.unwrap()).unwrap()
				&& *x <= mean.unwrap().checked_add(sd.unwrap()).unwrap()
			{
				new_items.push(*x);
			}
		}
		let new_mean = Self::mean_integer(&new_items);
		new_mean
	}
}
