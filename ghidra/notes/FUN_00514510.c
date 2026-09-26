
int __cdecl FUN_00514510(uint *param_1,uint *param_2,uint *param_3,int param_4,void *param_5)

{
  uint *puVar1;
  uint *puVar2;
  bool bVar3;
  int *piVar4;
  void *pvVar5;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int *piVar6;
  int iVar7;
  uint uVar8;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int iVar9;
  bool bVar10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puVar1 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006425d0;
  local_c = ExceptionList;
  iVar9 = 1;
  ExceptionList = &local_c;
  piVar4 = (int *)FUN_00504dc0(param_1);
  if (piVar4 != (int *)0x0) {
    pvVar5 = (void *)thunk_FUN_00506e40();
    puVar2 = param_2;
    bVar10 = pvVar5 != (void *)0x0;
    if (pvVar5 != (void *)0x0) {
      bVar3 = FUN_00545540(pvVar5,puVar1,param_2,param_3,param_4,param_5);
      if ((CONCAT31(extraout_var,bVar3) == 0) || (!bVar10)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
    }
    pvVar5 = (void *)thunk_FUN_00506e80();
    if ((pvVar5 == (void *)0x0) || (!bVar10)) {
      bVar10 = false;
    }
    else {
      bVar10 = true;
    }
    if (pvVar5 != (void *)0x0) {
      bVar3 = FUN_00561340(pvVar5,puVar1);
      if ((CONCAT31(extraout_var_00,bVar3) == 0) || (!bVar10)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
    }
    piVar6 = (int *)FUN_00504dc0(puVar2);
    if ((piVar6 == (int *)0x0) || (!bVar10)) {
      iVar9 = 0;
    }
    else {
      iVar9 = 1;
    }
    if (piVar6 != (int *)0x0) {
      iVar7 = (**(code **)(*piVar6 + 200))(piVar4,param_4,param_5);
      if ((iVar7 == 0) || (iVar9 == 0)) {
        iVar9 = 0;
      }
      else {
        iVar9 = 1;
      }
    }
    puVar1 = param_3;
    piVar6 = (int *)0x0;
    iVar7 = FUN_004ece60(param_3);
    if (iVar7 != 0) {
      piVar6 = (int *)FUN_00504dc0(puVar1);
    }
    if (piVar6 != (int *)0x0) {
      iVar7 = (**(code **)(*piVar6 + 0xd4))(piVar4,param_4,param_5);
      if ((iVar7 == 0) || (iVar9 == 0)) {
        iVar9 = 0;
      }
      else {
        iVar9 = 1;
      }
    }
    uStack_4 = 0;
    uVar8 = (**(code **)(*piVar4 + 4))();
    if ((uVar8 < 0x10) || (0x3f < uVar8)) {
      bVar10 = false;
    }
    else {
      bVar10 = true;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if (bVar10) {
      param_1 = (uint *)0x0;
      bVar10 = FUN_00518750((int)piVar4,(int *)&param_1);
      if ((CONCAT31(extraout_var_01,bVar10) == 0) || (iVar9 == 0)) {
        iVar9 = 0;
      }
      else {
        iVar9 = 1;
      }
      if (param_1 != (uint *)0x0) {
        iVar7 = FUN_0052fff0(param_1,param_5);
        if ((iVar7 == 0) || (iVar9 == 0)) {
          iVar9 = 0;
        }
        else {
          iVar9 = 1;
        }
      }
      bVar10 = true;
      uStack_4 = 1;
      uVar8 = (**(code **)(*piVar4 + 4))();
      if ((uVar8 < 0x20) || (0x2f < uVar8)) {
        bVar10 = false;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar10) {
        uStack_4 = 2;
        uVar8 = (**(code **)(*piVar4 + 4))();
        if ((uVar8 < 0x20) || (0x21 < uVar8)) {
          bVar10 = false;
        }
        else {
          bVar10 = true;
        }
        uStack_4 = 0xffffffff;
        FUN_00619730();
        if (bVar10) {
          bVar10 = FUN_00517de0((int)piVar4,param_4,param_3,param_5);
          if ((CONCAT31(extraout_var_02,bVar10) == 0) || (iVar9 == 0)) {
            iVar9 = 0;
          }
          else {
            iVar9 = 1;
          }
        }
      }
      uStack_4 = 3;
      uVar8 = (**(code **)(*piVar4 + 4))();
      if ((uVar8 < 0x18) || (0x1b < uVar8)) {
        bVar10 = false;
      }
      else {
        bVar10 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar10) {
        iVar7 = FUN_005178b0(param_5);
        if ((iVar7 == 0) || (iVar9 == 0)) {
          iVar9 = 0;
        }
        else {
          iVar9 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar9;
}

