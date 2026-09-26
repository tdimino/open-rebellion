
int * __fastcall FUN_004bbba0(int *param_1)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  
  piVar3 = (int *)0x0;
  FUN_0041b600((void *)param_1[0x11],param_1[6]);
  if (((uint)param_1[0xb] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xb] >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar2 = FUN_004f5940((void *)param_1[0x10],(uint *)(param_1 + 0xb));
    if ((pvVar2 != (void *)0x0) && (*(int *)((int)pvVar2 + 0x28) == param_1[6])) {
      *(uint *)((int)pvVar2 + 0x30) = *(uint *)((int)pvVar2 + 0x30) | 0x800000;
      *(undefined4 *)((int)pvVar2 + 0x2c) = 0;
      if (((*(uint *)((int)pvVar2 + 0x30) & 0x7000000) != 0) &&
         ((*(uint *)((int)pvVar2 + 0x30) & 0xa0) == 0)) {
        piVar3 = FUN_004bbf50(param_1,0x260);
      }
    }
  }
  (**(code **)(*param_1 + 0x2c))();
  FUN_004ece80(param_1 + 10);
  return piVar3;
}

