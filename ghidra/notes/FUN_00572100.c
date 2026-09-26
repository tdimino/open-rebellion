
uint __cdecl FUN_00572100(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  bool bVar1;
  void *this;
  undefined4 *puVar2;
  uint *puVar3;
  int iVar4;
  uint uVar5;
  undefined1 local_40 [4];
  undefined1 local_3c [4];
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_3;
  puStack_8 = &LAB_0064c3e0;
  local_c = ExceptionList;
  uVar5 = 1;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  param_3 = (void *)thunk_FUN_005f5060(param_1);
  do {
    if (((param_3 == (void *)0x0) || (uVar5 == 0)) || (*(int *)((int)this + 4) != -1)) {
      ExceptionList = local_c;
      return uVar5;
    }
    puVar3 = FUN_00403040(param_3,local_40);
    local_14 = 0x32;
    local_10 = 0x33;
    local_4 = 1;
    if ((*puVar3 >> 0x18 < 0x32) || (0x32 < *puVar3 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar1) {
      local_2c = 0x40;
      local_28 = 7;
      local_4 = 2;
      FUN_00520580(this,&local_2c);
      local_4 = 0xffffffff;
      FUN_00619730();
      *param_4 = 0xffffffff;
    }
    puVar2 = param_4;
    if (*(int *)((int)this + 4) == -1) {
      puVar3 = FUN_00403040(param_3,local_3c);
      local_4 = 3;
      iVar4 = FUN_00582cd0(puVar3,&local_34,&local_38,&param_1);
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar5 = 0;
      if (iVar4 != 0) {
        uVar5 = (uint)(param_1 != 0);
        goto LAB_00572278;
      }
    }
    else {
LAB_00572278:
      if (uVar5 != 0) {
        if ((*(int *)((int)this + 4) == -1) && ((*(byte *)(param_1 + 0xac) & 1) != 0)) {
          local_24 = 0x40;
          local_20 = 9;
          local_4 = 4;
          FUN_00520580(this,&local_24);
          local_4 = 0xffffffff;
          FUN_00619730();
          *puVar2 = 0xffffffff;
        }
        if (((uVar5 != 0) && (*(int *)((int)this + 4) == -1)) &&
           ((*(byte *)(param_1 + 0x78) & 1) != 0)) {
          local_1c = 0x40;
          local_18 = 8;
          local_4 = 5;
          FUN_00520580(this,&local_1c);
          local_4 = 0xffffffff;
          FUN_00619730();
          *puVar2 = 0xffffffff;
        }
      }
    }
    param_3 = *(void **)((int)param_3 + 0x10);
  } while( true );
}

