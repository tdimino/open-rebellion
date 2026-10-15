
void __thiscall FUN_0042ef90(void *this,uint param_1)

{
  int *piVar1;
  int iVar2;
  void *this_00;
  
  if (param_1 != 0) {
    piVar1 = (int *)FUN_005f5500(this,param_1);
    if (piVar1 != (int *)0x0) {
      iVar2 = (**(code **)(*piVar1 + 0xc))();
      if (iVar2 == 1) {
        this_00 = (void *)FUN_005f5500((void *)(*(int *)((int)this + 0x14) + 0xd8),piVar1[0x15]);
        if (this_00 != (void *)0x0) {
          FUN_00476de0(this_00,piVar1);
        }
      }
      piVar1[8] = piVar1[8] | 0x80000000;
    }
  }
  return;
}

