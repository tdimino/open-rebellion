
undefined4 __thiscall FUN_005aada0(void *this,int *param_1)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  
  FUN_005ca4c0((void *)((int)this + 0xa44),(int)param_1);
  uVar3 = param_1[6];
  pvVar1 = (void *)FUN_00596bd0();
  iVar2 = FUN_005a6480(pvVar1,uVar3);
  if (iVar2 == 0) {
    piVar4 = param_1;
    pvVar1 = (void *)FUN_00596bd0();
    FUN_005a6540(pvVar1,piVar4);
  }
  iVar2 = (**(code **)(*(int *)(*(int *)((int)this + param_1[0x14] * 4 + 8) + 0x70) + 0x14))
                    (param_1[6]);
  if (iVar2 == 0) {
    FUN_005ca4c0((void *)(*(int *)((int)this + param_1[0x14] * 4 + 8) + 0x70),(int)param_1);
  }
  return 1;
}

