
void __thiscall FUN_005be350(void *this,int param_1)

{
  FUN_00403e90(*(void **)((int)this + 8),0);
  FUN_00403e90(*(void **)((int)this + 0xc),0);
  FUN_005c7e10(*(void **)((int)this + 8),(uint)(param_1 != 0));
  FUN_005c7e10(*(void **)((int)this + 0xc),(uint)(param_1 != 1));
  FUN_005ab650(this);
  if (*(int *)((int)this + 0x8e8) == 1) {
    FUN_005ae1e0();
                    /* WARNING: Load size is inaccurate */
    (*(code *)**this)();
    return;
  }
  FUN_005ae1e0();
                    /* WARNING: Load size is inaccurate */
  (*(code *)**this)();
  return;
}

