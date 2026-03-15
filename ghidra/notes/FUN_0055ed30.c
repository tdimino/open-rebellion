
int FUN_0055ed30(void *param_1,int *param_2,int *param_3,undefined4 *param_4)

{
  void *pvVar1;
  int *piVar2;
  bool bVar3;
  undefined3 extraout_var;
  uint uVar5;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  int iVar4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649f10;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  param_2 = (int *)0x0;
  bVar3 = FUN_004f6b50(param_1,(int *)&param_2);
  iVar4 = CONCAT31(extraout_var,bVar3);
  if (iVar4 != 0) {
    local_4 = 0;
    uVar5 = (**(code **)(*param_2 + 4))();
    if ((uVar5 < 0x14) || (0x1b < uVar5)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar3) {
      *piVar2 = (int)param_2;
    }
    else {
      bVar3 = true;
      local_4 = 1;
      uVar5 = (**(code **)(*param_2 + 4))();
      if ((uVar5 < 0x90) || (0x97 < uVar5)) {
        bVar3 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar3) {
        *param_4 = param_2;
      }
    }
    if (iVar4 != 0) {
      pvVar1 = (void *)*piVar2;
      if (pvVar1 != (void *)0x0) {
        param_2 = (int *)0x0;
        bVar3 = FUN_004f6b50(pvVar1,(int *)&param_2);
        iVar4 = CONCAT31(extraout_var_00,bVar3);
        if (iVar4 == 0) {
          ExceptionList = local_c;
          return 0;
        }
        local_4 = 2;
        uVar5 = (**(code **)(*param_2 + 4))();
        if ((uVar5 < 8) || (0xf < uVar5)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar3) {
          *param_3 = (int)param_2;
        }
      }
      if ((iVar4 != 0) && (pvVar1 = (void *)*param_3, pvVar1 != (void *)0x0)) {
        param_3 = (int *)0x0;
        bVar3 = FUN_004f6b50(pvVar1,(int *)&param_3);
        iVar4 = CONCAT31(extraout_var_01,bVar3);
        if (iVar4 != 0) {
          local_4 = 3;
          uVar5 = (**(code **)(*param_3 + 4))();
          if ((uVar5 < 0x90) || (0x97 < uVar5)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          local_4 = 0xffffffff;
          FUN_00619730();
          if (bVar3) {
            *param_4 = param_3;
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

