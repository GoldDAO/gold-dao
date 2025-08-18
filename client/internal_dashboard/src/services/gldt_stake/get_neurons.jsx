const get_neurons = async (actor) => {
  const result = await actor.get_neurons(null);
  return result;
};

export default get_neurons;
