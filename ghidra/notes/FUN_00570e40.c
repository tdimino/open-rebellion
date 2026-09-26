
uint __cdecl FUN_00570e40(int param_1,int param_2,int *param_3)

{
  bool bVar1;
  void *this;
  uint *puVar2;
  void *this_00;
  uint uVar3;
  undefined1 local_1c [4];
  undefined1 local_18 [4];
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c1a0;
  local_c = ExceptionList;
  uVar3 = 1;
  ExceptionList = &local_c;
  *param_3 = 0;
  this = (void *)thunk_FUN_005f5060(param_2);
  do {
    if ((this == (void *)0x0) || (*param_3 != 0)) break;
    puVar2 = FUN_00403040(this,local_1c);
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
      puVar2 = FUN_00403040(this,local_18);
      local_4 = 1;
      this_00 = (void *)FUN_00505190(puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar3 = (uint)(this_00 != (void *)0x0);
      param_2 = 0;
      if (uVar3 != 0) {
        uVar3 = FUN_004ed310(this_00,&param_2);
        if ((uVar3 != 0) && (param_2 != 0)) {
          *param_3 = (int)this_00;
        }
      }
    }
    this = *(void **)((int)this + 0x10);
  } while (uVar3 != 0);
  if ((((uVar3 != 0) && (*param_3 != 0)) && (param_1 != 1)) && (param_1 != 2)) {
    uVar3 = 0;
  }
  ExceptionList = local_c;
  return uVar3;
}

