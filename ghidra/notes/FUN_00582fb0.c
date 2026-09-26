
int __cdecl
FUN_00582fb0(int *param_1,undefined4 param_2,undefined4 param_3,uint *param_4,void *param_5)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064f050;
  local_c = ExceptionList;
  bVar1 = true;
  iVar4 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_5,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((param_1[9] & 0xc0U) == 0x40) {
    param_4[2] = 1;
  }
  else if ((param_1[9] & 0xc0U) == 0x80) {
    param_4[3] = 1;
  }
  else {
    iVar4 = 0;
  }
  if (iVar4 != 0) {
    local_14 = 0x3c;
    local_10 = 0x40;
    local_4 = 1;
    uVar2 = (**(code **)(*param_1 + 4))();
    if ((uVar2 < local_14) || (local_10 <= uVar2)) {
      bVar1 = false;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      uVar2 = *param_4;
      uVar3 = FUN_00503b40((int)param_1);
      *param_4 = uVar3 | uVar2;
    }
    else {
      uVar2 = param_4[1];
      uVar3 = FUN_004ed260();
      param_4[1] = uVar3 | uVar2;
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

