
void __thiscall FUN_005a04f0(void *this,int param_1)

{
  uint uVar1;
  void **ppvVar2;
  undefined4 *puVar3;
  int *piVar4;
  int iVar5;
  int iVar6;
  void *pvVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651d5c;
  local_c = ExceptionList;
  puVar3 = (undefined4 *)0x0;
  ppvVar2 = &local_c;
  if (param_1 != 0) {
    uVar1 = *(uint *)(param_1 + 0xc);
    if (uVar1 == 0) {
      if (*(int *)(param_1 + 0x10) != 0) {
        return;
      }
      ExceptionList = &local_c;
      puVar3 = (undefined4 *)(**(code **)(*DAT_006bc464 + 0x18))();
      ppvVar2 = ExceptionList;
    }
    else {
      ExceptionList = &local_c;
      if (DAT_006bc470 == (undefined4 *)0x0) {
        ExceptionList = &local_c;
        puVar3 = (undefined4 *)FUN_00618b70(0x48);
        local_4 = 0;
        if (puVar3 == (undefined4 *)0x0) {
          DAT_006bc470 = (undefined4 *)0x0;
        }
        else {
          DAT_006bc470 = FUN_005a6290(puVar3);
        }
      }
      local_4 = 0xffffffff;
      piVar4 = (int *)FUN_005a63e0(DAT_006bc470,uVar1);
      iVar5 = (**(code **)(*piVar4 + 4))();
      if (iVar5 == 1) {
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + -0x35;
        }
        if (*(int *)this != 0) {
          puVar3 = (undefined4 *)
                   (**(code **)(*DAT_006bc464 + 0xc))(uVar1,*(undefined4 *)(param_1 + 0x10));
          ppvVar2 = ExceptionList;
          goto LAB_005a066a;
        }
        iVar5 = piVar4[0x5a];
        iVar6 = FUN_005aaf90();
        if (iVar5 == *(int *)(iVar6 + 0x9ec)) {
          pvVar7 = (void *)FUN_00618b70(0x54);
          local_4 = 1;
          if (pvVar7 != (void *)0x0) {
            puVar3 = FUN_005a1550(pvVar7,uVar1);
            local_4 = 0xffffffff;
            ppvVar2 = ExceptionList;
            goto LAB_005a066a;
          }
        }
        else {
          pvVar7 = (void *)FUN_00618b70(0x54);
          local_4 = 2;
          if (pvVar7 != (void *)0x0) {
            puVar3 = FUN_005a1110(pvVar7,uVar1);
            local_4 = 0xffffffff;
            ppvVar2 = ExceptionList;
            goto LAB_005a066a;
          }
        }
      }
      else {
        if (*(int *)this != 0) {
          puVar3 = (undefined4 *)
                   (**(code **)(*DAT_006bc464 + 0x10))(uVar1,*(undefined4 *)(param_1 + 0x10));
          ppvVar2 = ExceptionList;
          goto LAB_005a066a;
        }
        pvVar7 = (void *)FUN_00618b70(0x54);
        local_4 = 3;
        if (pvVar7 != (void *)0x0) {
          puVar3 = FUN_005a0e70(pvVar7,uVar1);
          local_4 = 0xffffffff;
          ppvVar2 = ExceptionList;
          goto LAB_005a066a;
        }
      }
      puVar3 = (undefined4 *)0x0;
      local_4 = 0xffffffff;
      ppvVar2 = ExceptionList;
    }
  }
LAB_005a066a:
  ExceptionList = ppvVar2;
  FUN_0059f350(puVar3);
  ExceptionList = local_c;
  return;
}

