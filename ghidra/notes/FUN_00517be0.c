
bool __cdecl FUN_00517be0(void *param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  bool bVar3;
  uint local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006429b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_005180e0(param_4);
  bVar3 = iVar2 != 0;
  if (((*param_3 ^ *param_2) & 3) != 0) {
    FUN_00562ae0(local_28);
    local_4 = 0;
    iVar2 = FUN_0055e010(param_2,param_3,local_28);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    bVar1 = FUN_00518ac0(param_1,local_28,param_4);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    local_4 = 0xffffffff;
    FUN_00562b30();
  }
  ExceptionList = local_c;
  return bVar3;
}

