
uint __cdecl FUN_00575ae0(int param_1,undefined4 param_2,void *param_3,undefined4 *param_4)

{
  int iVar1;
  void *this;
  uint *puVar2;
  int *piVar3;
  uint uVar4;
  int local_34;
  undefined1 local_30 [4];
  uint local_2c;
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
  
  puStack_8 = &LAB_0064c9b8;
  local_c = ExceptionList;
  uVar4 = 1;
  local_24 = 1;
  local_20 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  this = (void *)thunk_FUN_005f5060(param_1);
  do {
    if (((this == (void *)0x0) || (uVar4 == 0)) || (*(int *)((int)param_3 + 4) != -1)) {
      ExceptionList = local_c;
      return uVar4;
    }
    puVar2 = (uint *)FUN_00403040(this,local_30);
    local_4 = 1;
    uVar4 = FUN_00582cd0(puVar2,&local_24,&local_34,&local_28);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar4 != 0) {
      uVar4 = (uint)(local_34 != 0);
    }
    if (uVar4 != 0) {
      if (*(int *)((int)param_3 + 4) == -1) {
        uVar4 = (uint)(*(void **)(local_34 + 0x2c) != (void *)0x0);
        if (uVar4 == 0) goto LAB_00575c98;
        local_4 = 2;
        piVar3 = (int *)FUN_00402e40(*(void **)(local_34 + 0x2c),&local_2c);
        iVar1 = *piVar3;
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if (iVar1 != 0x3c000009) {
          local_1c = 0x40;
          local_18 = 0xb;
          local_4 = 3;
          FUN_00520580(param_3,&local_1c);
          local_4 = 0xffffffff;
          FUN_00619730();
          *param_4 = 0xffffffff;
        }
      }
      if (((uVar4 != 0) && (*(int *)((int)param_3 + 4) == -1)) &&
         ((*(byte *)(local_34 + 0x78) & 1) != 0)) {
        local_14 = 0x40;
        local_10 = 8;
        local_4 = 4;
        FUN_00520580(param_3,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *param_4 = 0xffffffff;
      }
    }
LAB_00575c98:
    this = *(void **)((int)this + 0x10);
  } while( true );
}

