const get_total_staked = async (actor) => {
  const result = await actor.get_total_staked(null);
  return result;
};

export default get_total_staked;
