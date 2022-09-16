#include <internal.h>
#include <inttypes.h>
#include <stdlib.h>

int loaded_modules = 0;
struct Module_Entry *modules = NULL;

struct Module_Entry
{
    uint32_t module_id;
    Function_Handler fn;
    Module_Load load;
};

void register_module(uint32_t module_id, Module_Load load, Function_Handler fn)
{
    loaded_modules++;
    modules = reallocarray(modules, loaded_modules, sizeof(struct Module_Entry));
    modules[loaded_modules - 1] = (struct Module_Entry){
        .fn = fn,
        .load = load,
        .module_id = module_id};
}

void call_module(uint32_t module_id, struct packet_list_t *packet)
{
    for (int i = 0; i < loaded_modules; i++)
    {
        if (modules[i].module_id == module_id)
        {
            modules[i].fn(packet);
            return;
        }
    }
}
void init_modules(void)
{
    for (int i = 0; i < loaded_modules; i++)
    {
        modules[i].load();
    }
}
