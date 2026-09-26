
void __thiscall FUN_005a5fd0(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  void *pvVar3;
  undefined4 uVar4;
  int *piVar5;
  undefined2 extraout_var;
  uint uVar6;
  undefined2 extraout_var_00;
  undefined1 uVar7;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006527d6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_005aaf90();
  for (iVar1 = (**(code **)(*(int *)(iVar1 + 0xe50) + 0xc))(); iVar1 != 0;
      iVar1 = *(int *)(iVar1 + 0x10)) {
    iVar2 = FUN_00583c40((int)this);
    if ((iVar2 != 0) && (*(int *)(iVar2 + 0xec) == *(int *)(iVar1 + 0x5c))) {
      pvVar3 = (void *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        uVar6 = CONCAT22(extraout_var,*(undefined2 *)(iVar1 + 0x34));
        uVar7 = 2;
        uVar4 = FUN_005fefd0(6);
        pvVar3 = FUN_005fbd20(pvVar3,uVar4,uVar6,uVar7);
      }
      uStack_4 = 0xffffffff;
      FUN_00609840(param_1,*(undefined4 *)(iVar1 + 0x18),(int)pvVar3,0,&DAT_006b120c,0,
                   *(undefined4 *)(iVar1 + 0x18));
    }
  }
  iVar1 = FUN_005aaf90();
  for (iVar1 = (**(code **)(*(int *)(iVar1 + 0xe5c) + 0xc))(); iVar1 != 0;
      iVar1 = *(int *)(iVar1 + 0x10)) {
    iVar2 = FUN_00583c40((int)this);
    if ((iVar2 != 0) && (*(int *)(iVar2 + 0xec) == *(int *)(iVar1 + 0x48))) {
      pvVar3 = (void *)FUN_00618b70(0x28);
      uStack_4 = 1;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        uVar6 = CONCAT22(extraout_var_00,*(undefined2 *)(iVar1 + 0x34));
        uVar7 = 2;
        uVar4 = FUN_005fefd0(6);
        pvVar3 = FUN_005fbd20(pvVar3,uVar4,uVar6,uVar7);
      }
      uStack_4 = 0xffffffff;
      FUN_00609840(param_1,*(undefined4 *)(iVar1 + 0x18),(int)pvVar3,0,&DAT_006b120c,0,
                   *(undefined4 *)(iVar1 + 0x18));
    }
  }
  for (piVar5 = (int *)FUN_00609eb0((int)param_1); piVar5 != (int *)0x0;
      piVar5 = (int *)(**(code **)(*piVar5 + 0xc))()) {
    piVar5[0xf] = piVar5[0xf] | 4;
  }
  ExceptionList = pvStack_c;
  return;
}

