
undefined4 __cdecl FUN_005140c0(uint *param_1,uint *param_2,uint *param_3,int param_4,void *param_5)

{
  uint *puVar1;
  bool bVar2;
  int *piVar3;
  void *pvVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar5;
  int *piVar6;
  uint uVar7;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined4 uVar8;
  bool bVar9;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006425a0;
  local_c = ExceptionList;
  uVar8 = 1;
  ExceptionList = &local_c;
  piVar3 = (int *)FUN_00504dc0(param_1);
  if (piVar3 != (int *)0x0) {
    pvVar4 = (void *)thunk_FUN_00506e40();
    puVar1 = param_2;
    bVar9 = pvVar4 != (void *)0x0;
    if (pvVar4 != (void *)0x0) {
      bVar2 = FUN_00545240(pvVar4,param_1,param_2,(int *)param_3,param_4,param_5);
      if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    pvVar4 = (void *)thunk_FUN_00506e80();
    if ((pvVar4 == (void *)0x0) || (!bVar9)) {
      bVar9 = false;
    }
    else {
      bVar9 = true;
    }
    if (pvVar4 != (void *)0x0) {
      bVar2 = FUN_00561470(pvVar4,param_1,puVar1,param_3,param_4,param_5);
      if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    if ((*(byte *)(piVar3 + 0x14) & 8) != 0) {
      iVar5 = FUN_005175e0(piVar3,param_5);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    if ((*(byte *)(piVar3 + 0x14) & 4) == 0) {
      iVar5 = FUN_004ece60((uint *)(piVar3 + 0xe));
      if (iVar5 != 0) {
        pvVar4 = (void *)FUN_00505750((uint *)(piVar3 + 0xe));
        if ((pvVar4 == (void *)0x0) || (!bVar9)) {
          bVar9 = false;
        }
        else {
          bVar9 = true;
        }
        if (pvVar4 != (void *)0x0) {
          iVar5 = FUN_00529cf0(pvVar4,param_1);
          if ((iVar5 == 0) || (!bVar9)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
        }
      }
    }
    piVar6 = (int *)FUN_00504dc0(puVar1);
    if ((piVar6 == (int *)0x0) || (!bVar9)) {
      bVar9 = false;
    }
    else {
      bVar9 = true;
    }
    if (piVar6 != (int *)0x0) {
      iVar5 = (**(code **)(*piVar6 + 0xcc))(piVar3,param_4,param_5);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    if (piVar6 != (int *)0x0) {
      iVar5 = (**(code **)(*piVar6 + 0xc4))(piVar3,param_4,param_5);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    puVar1 = param_3;
    piVar6 = (int *)0x0;
    iVar5 = FUN_004ece60(param_3);
    if (iVar5 != 0) {
      piVar6 = (int *)FUN_00504dc0(puVar1);
    }
    if (piVar6 != (int *)0x0) {
      iVar5 = (**(code **)(*piVar6 + 0xd0))(piVar3,param_4,param_5);
      if ((iVar5 == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    uVar7 = FUN_00517760(piVar3,param_5);
    if ((uVar7 == 0) || (!bVar9)) {
      bVar9 = false;
    }
    else {
      bVar9 = true;
    }
    uStack_4 = 0;
    uVar7 = (**(code **)(*piVar3 + 4))();
    if ((uVar7 < 0x10) || (0x3f < uVar7)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      param_2 = (uint *)0x0;
      bVar2 = FUN_00518750((int)piVar3,(int *)&param_2);
      if ((CONCAT31(extraout_var_01,bVar2) == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
      if (param_2 != (uint *)0x0) {
        iVar5 = FUN_0052fff0(param_2,param_5);
        if ((iVar5 == 0) || (!bVar9)) {
          bVar9 = false;
        }
        else {
          bVar9 = true;
        }
        if ((param_2 != (uint *)0x0) && ((*(byte *)(piVar3 + 0x14) & 8) != 0)) {
          iVar5 = FUN_00530270(param_2,(int)piVar3,param_5);
          if ((iVar5 == 0) || (!bVar9)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
        }
      }
      uStack_4 = 1;
      uVar7 = (**(code **)(*piVar3 + 4))();
      if ((uVar7 < 0x20) || (0x2f < uVar7)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar2) {
        uStack_4 = 2;
        uVar7 = (**(code **)(*piVar3 + 4))();
        if ((uVar7 < 0x20) || (0x21 < uVar7)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        uStack_4 = 0xffffffff;
        FUN_00619730();
        if (bVar2) {
          iVar5 = FUN_00517cd0(piVar3,param_4,param_3,param_5);
          if ((iVar5 == 0) || (!bVar9)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
        }
      }
    }
    uStack_4 = 3;
    uVar7 = (**(code **)(*piVar3 + 4))();
    if ((uVar7 < 0x90) || (0x97 < uVar7)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if (bVar2) {
      bVar2 = FUN_00517bb0((int)piVar3,param_4,param_5);
      if ((CONCAT31(extraout_var_02,bVar2) == 0) || (!bVar9)) {
        bVar9 = false;
      }
      else {
        bVar9 = true;
      }
    }
    uVar7 = FUN_00517510(piVar3,param_5);
    if ((uVar7 == 0) || (!bVar9)) {
      uVar8 = 0;
    }
    else {
      uVar8 = 1;
    }
  }
  ExceptionList = local_c;
  return uVar8;
}

