
uint __thiscall FUN_0052a140(void *this,int param_1,undefined4 param_2,void *param_3)

{
  void *this_00;
  uint *puVar1;
  void *this_01;
  int *this_02;
  uint uVar2;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006448f0;
  local_c = ExceptionList;
  uVar2 = 1;
  ExceptionList = &local_c;
  this_00 = (void *)thunk_FUN_005f5060(param_1);
  do {
    if (this_00 == (void *)0x0) {
      ExceptionList = local_c;
      return uVar2;
    }
    puVar1 = FUN_00403040(this_00,&param_1);
    local_4 = 0;
    this_01 = (void *)FUN_00504dc0(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar2 = 0;
    if (this_01 != (void *)0x0) {
      this_02 = FUN_004f7d50(this_01,(uint *)((int)this_00 + 0x1c),
                             *(uint *)((int)this + 0x24) >> 6 & 3,param_3,1,0);
      uVar2 = 0;
      if (this_02 != (int *)0x0) {
        puVar1 = FUN_004025b0(this,&local_10);
        local_4 = 1;
        uVar2 = FUN_004f7070(this_02,(int *)puVar1,param_3);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
    this_00 = *(void **)((int)this_00 + 0x10);
  } while (uVar2 != 0);
  ExceptionList = local_c;
  return 0;
}

