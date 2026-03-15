
void __thiscall FUN_0059fbd0(void *this,int param_1)

{
  int *piVar1;
  int iVar2;
  
  *(uint *)((int)this + param_1 * 4 + 0x144) =
       (uint)(*(int *)((int)this + param_1 * 4 + 0x144) == 0);
  piVar1 = (int *)(**(code **)(*(int *)((int)this + (param_1 + 0x17) * 0xc) + 0xc))();
  (**(code **)(*piVar1 + 8))();
  for (; piVar1 != (int *)0x0; piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    iVar2 = (**(code **)(*piVar1 + 8))();
    if (*(int *)((int)this + param_1 * 4 + 0x144) == 0) {
      *(undefined4 *)(iVar2 + 0x34) = 1;
    }
    else {
      *(undefined4 *)(iVar2 + 0x34) = 0;
    }
    FUN_0059ed70();
    FUN_0059f560(iVar2);
  }
  return;
}

