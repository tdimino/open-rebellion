
int __fastcall FUN_004cd040(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b6d0;
  local_c = ExceptionList;
  this = (uint *)(param_1 + 0x5c);
  local_1c = 0;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 0x68) = 0;
  FUN_004ece80(this);
  pvVar3 = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x44))
  ;
  if ((pvVar3 != (void *)0x0) ||
     (pvVar3 = (void *)thunk_FUN_005f5060(*(int *)(param_1 + 0x2c) + 0xd8), pvVar3 != (void *)0x0))
  {
    do {
      if (local_1c != 0) break;
      puVar4 = (undefined4 *)FUN_00476f90(pvVar3,&local_18);
      local_4 = 0;
      FUN_004f26d0(this,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      local_14 = 0x90;
      local_10 = 0x98;
      if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        local_1c = 1;
        *(undefined4 *)(param_1 + 0x68) = *(undefined4 *)((int)pvVar3 + 0x9c);
      }
      else {
        pvVar3 = *(void **)((int)pvVar3 + 0x10);
      }
    } while (pvVar3 != (void *)0x0);
    if (pvVar3 != (void *)0x0) {
      *(undefined4 *)(param_1 + 0x40) = *(undefined4 *)((int)pvVar3 + 0x18);
      if (*(int *)((int)pvVar3 + 0x10) != 0) {
        *(undefined4 *)(param_1 + 0x44) = *(undefined4 *)(*(int *)((int)pvVar3 + 0x10) + 0x18);
        ExceptionList = local_c;
        return local_1c;
      }
      *(undefined4 *)(param_1 + 0x44) = 0;
      ExceptionList = local_c;
      return local_1c;
    }
  }
  *(undefined4 *)(param_1 + 0x40) = 0;
  *(undefined4 *)(param_1 + 0x44) = 0;
  pvVar5 = FUN_0041a4c0(*(void **)(param_1 + 0x2c),0x100005,0xc02,0x17,&DAT_00000002);
  pvVar3 = (void *)(param_1 + 0x48);
  FUN_00435790(pvVar3,(int)pvVar5);
  puVar4 = (undefined4 *)FUN_004357b0(pvVar3,&local_18);
  local_4 = 1;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar3);
  local_14 = 8;
  local_10 = 0x10;
  if ((*this >> 0x18 < 8) || (0xf < *this >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x2c);
    local_18 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,this);
    pvVar3 = FUN_00419e40((void *)(iVar1 + 0x78));
    if (pvVar3 != (void *)0x0) {
      *(undefined4 *)(param_1 + 0x68) = 1;
      local_1c = 1;
    }
  }
  ExceptionList = local_c;
  return local_1c;
}

