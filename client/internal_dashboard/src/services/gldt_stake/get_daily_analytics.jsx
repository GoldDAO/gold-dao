const get_daily_analytics = async (actor) => {
  const result = await actor.get_daily_analytics({
    starting_day: 0,
    limit: [],
  });
  return result;
};

export default get_daily_analytics;
