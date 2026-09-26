
bool __cdecl FUN_00517aa0(void *param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  void *this;
  int iVar2;
  bool bVar3;
  uint local_28 [6];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642998;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00506e10();
  bVar3 = this != (void *)0x0;
  if (((*param_2 ^ *param_3) & 3) != 0) {
    FUN_00562ae0(local_28);
    local_4 = 0;
    iVar2 = FUN_0055de50(param_2,param_3,local_28);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    if (local_10 != 0) {
      iVar2 = 1;
      if (((*param_2 & 0x30) == 0) && ((*param_3 & 0x30) == 0)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      if ((this != (void *)0x0) && (bVar1)) {
        iVar2 = FUN_00518040(this,param_2);
      }
      if (iVar2 != 0) {
        bVar3 = FUN_00518ac0(param_1,local_28,param_4);
      }
    }
    local_4 = 0xffffffff;
    FUN_00562b30();
  }
  if (*(int *)((int)param_1 + 0x18) == 0x109) {
    bVar3 = FUN_00517be0(param_1,param_2,param_3,param_4);
  }
  ExceptionList = local_c;
  return bVar3;
}

