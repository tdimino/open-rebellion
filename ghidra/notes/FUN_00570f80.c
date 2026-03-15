
bool __cdecl FUN_00570f80(int param_1,int param_2,int *param_3)

{
  bool bVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  undefined1 local_18 [4];
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c1c0;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  *param_3 = 0;
  this = (void *)thunk_FUN_005f5060(param_2);
  do {
    if ((this == (void *)0x0) || (*param_3 != 0)) break;
    puVar2 = (uint *)FUN_00559950(this,&param_2);
    local_14 = 0x30;
    local_10 = 0x3c;
    local_4 = 0;
    if ((*puVar2 >> 0x18 < 0x30) || (0x3b < *puVar2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      puVar2 = (uint *)FUN_00559950(this,local_18);
      local_4 = 1;
      iVar3 = FUN_0051cab0(puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      bVar4 = iVar3 != 0;
      if (((bVar4) && (bVar4)) && (*(int *)(iVar3 + 0xb8) != 0)) {
        *param_3 = iVar3;
      }
    }
    this = *(void **)((int)this + 0x10);
  } while (bVar4);
  if (((bVar4) && (*param_3 != 0)) && ((param_1 != 1 && (param_1 != 2)))) {
    bVar4 = false;
  }
  ExceptionList = local_c;
  return bVar4;
}

