
int __cdecl FUN_00516220(uint *param_1,undefined4 param_2,uint param_3,void *param_4)

{
  bool bVar1;
  int *piVar2;
  void *pvVar3;
  uint uVar4;
  undefined3 extraout_var;
  int iVar5;
  int iVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642788;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00505190(param_1);
  pvVar3 = (void *)thunk_FUN_00506e40();
  if ((pvVar3 == (void *)0x0) || (piVar2 == (int *)0x0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (pvVar3 != (void *)0x0) {
    uVar4 = FUN_00546370(pvVar3,param_1,param_2,param_3,param_4);
    if ((uVar4 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  pvVar3 = (void *)thunk_FUN_00506e80();
  if ((pvVar3 == (void *)0x0) || (!bVar1)) {
    iVar6 = 0;
  }
  else {
    iVar6 = 1;
  }
  if (pvVar3 != (void *)0x0) {
    bVar1 = FUN_005615b0(pvVar3,param_1,param_2,param_3,param_4);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (iVar6 == 0)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
  }
  if (piVar2 != (int *)0x0) {
    local_4 = 0;
    uVar4 = (**(code **)(*piVar2 + 4))();
    if ((uVar4 < 0x30) || (0x37 < uVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar5 = FUN_005180e0(param_4);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return iVar6;
}

