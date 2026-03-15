
int __thiscall
FUN_00587bb0(void *this,int *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            undefined4 param_7)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  undefined4 uVar3;
  int iVar4;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int iVar5;
  int *piVar6;
  undefined4 uVar7;
  int local_48;
  int local_44;
  void *local_40;
  undefined4 local_3c [11];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f8c0;
  pvStack_c = ExceptionList;
  iVar5 = 1;
  local_44 = 0;
  local_48 = 0;
  ExceptionList = &pvStack_c;
  local_40 = this;
  if ((param_3 != 0) && (ExceptionList = &pvStack_c, param_4 != 0)) {
    ExceptionList = &pvStack_c;
    FUN_00525e70(local_3c,*(undefined4 *)((int)this + 0xc));
    local_4 = 0;
    FUN_00525930((int)local_3c);
    while ((local_10 != 0 && (local_44 == 0))) {
      piVar6 = &local_48;
      iVar4 = param_6;
      uVar2 = FUN_00525f20((int)local_3c);
      bVar1 = FUN_005883b0(this,uVar2,iVar4,piVar6);
      if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if (local_48 != 0) {
        piVar6 = &local_44;
        iVar4 = *param_1;
        uVar7 = param_7;
        uVar3 = FUN_00525f20((int)local_3c);
        iVar4 = (**(code **)(iVar4 + 4))(uVar3,piVar6,uVar7);
        this = local_40;
        if ((iVar4 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
      }
      FUN_005258f0((int)local_3c);
    }
    local_4 = 0xffffffff;
    FUN_00525f10(local_3c);
  }
  bVar1 = iVar5 == 0;
  if (!bVar1) {
    if ((param_2 != 0) && (param_4 != 0)) {
      FUN_00526090(local_3c,*(undefined4 *)((int)this + 0xc));
      local_4 = 1;
      FUN_00525930((int)local_3c);
      while ((local_10 != 0 && (local_44 == 0))) {
        piVar6 = &local_48;
        iVar4 = param_6;
        uVar2 = FUN_00525f20((int)local_3c);
        bVar1 = FUN_005883b0(this,uVar2,iVar4,piVar6);
        if ((CONCAT31(extraout_var_00,bVar1) == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
        if (local_48 != 0) {
          piVar6 = &local_44;
          iVar4 = *param_1;
          uVar7 = param_7;
          uVar3 = FUN_00525f20((int)local_3c);
          iVar4 = (**(code **)(iVar4 + 4))(uVar3,piVar6,uVar7);
          if ((iVar4 == 0) || (iVar5 == 0)) {
            iVar5 = 0;
          }
          else {
            iVar5 = 1;
          }
        }
        FUN_005258f0((int)local_3c);
      }
      local_4 = 0xffffffff;
      FUN_00526130(local_3c);
    }
    bVar1 = iVar5 == 0;
  }
  if (((!bVar1) && (param_3 != 0)) && (param_5 != 0)) {
    FUN_00525f30(local_3c,*(undefined4 *)((int)this + 0xc));
    local_4 = 2;
    FUN_00525930((int)local_3c);
    while ((local_10 != 0 && (local_44 == 0))) {
      piVar6 = &local_48;
      iVar4 = param_6;
      uVar2 = FUN_00525f20((int)local_3c);
      bVar1 = FUN_005883b0(this,uVar2,iVar4,piVar6);
      if ((CONCAT31(extraout_var_01,bVar1) == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if (local_48 != 0) {
        piVar6 = &local_44;
        iVar4 = *param_1;
        uVar7 = param_7;
        uVar3 = FUN_00525f20((int)local_3c);
        iVar4 = (**(code **)(iVar4 + 4))(uVar3,piVar6,uVar7);
        this = local_40;
        if ((iVar4 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
      }
      FUN_005258f0((int)local_3c);
    }
    local_4 = 0xffffffff;
    FUN_00525fd0(local_3c);
  }
  if (((iVar5 != 0) && (param_2 != 0)) && (param_5 != 0)) {
    FUN_00526140(local_3c,*(undefined4 *)((int)this + 0xc));
    local_4 = 3;
    FUN_00525930((int)local_3c);
    while ((local_10 != 0 && (local_44 == 0))) {
      piVar6 = &local_48;
      iVar4 = param_6;
      uVar2 = FUN_00525f20((int)local_3c);
      bVar1 = FUN_005883b0(this,uVar2,iVar4,piVar6);
      if ((CONCAT31(extraout_var_02,bVar1) == 0) || (iVar5 == 0)) {
        iVar5 = 0;
      }
      else {
        iVar5 = 1;
      }
      if (local_48 != 0) {
        piVar6 = &local_44;
        iVar4 = *param_1;
        uVar7 = param_7;
        uVar3 = FUN_00525f20((int)local_3c);
        iVar4 = (**(code **)(iVar4 + 4))(uVar3,piVar6,uVar7);
        if ((iVar4 == 0) || (iVar5 == 0)) {
          iVar5 = 0;
        }
        else {
          iVar5 = 1;
        }
      }
      FUN_005258f0((int)local_3c);
    }
    local_4 = 0xffffffff;
    FUN_005261e0(local_3c);
  }
  ExceptionList = pvStack_c;
  return iVar5;
}

