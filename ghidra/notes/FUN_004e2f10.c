
int __fastcall FUN_004e2f10(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  int local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063dec0;
  local_c = ExceptionList;
  local_18 = 0;
  this = (uint *)(param_1 + 0x44);
  uVar3 = *(uint *)(param_1 + 0x44) >> 0x18;
  local_14 = (undefined1 *)0x90;
  local_10 = 0x98;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x2c);
    local_14 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,this);
    pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (((*(uint *)((int)pvVar4 + 0x28) & 3) == 0) && ((*(uint *)((int)pvVar4 + 0x28) & 0x100) != 0)
       ) {
      local_18 = 1;
    }
  }
  if (local_18 == 0) {
    puVar6 = (undefined4 *)(param_1 + 0x4c);
    puVar5 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar6,0x200,0,0,0,0,0,2);
    pvVar4 = (void *)(param_1 + 0x70);
    FUN_00435790(pvVar4,(int)puVar5);
    puVar5 = FUN_004357b0(pvVar4,&local_14);
    local_4 = 0;
    FUN_004f26d0(this,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    puVar5 = FUN_0041a400(*(void **)(param_1 + 0x2c),this,puVar6,0x100,0,0,3,0,0,
                          (undefined4 *)&DAT_00000002);
    FUN_00435790(pvVar4,(int)puVar5);
    puVar5 = FUN_004357b0(pvVar4,&local_14);
    local_4 = 1;
    FUN_004f26d0(this,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      puVar5 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar6,0x400,0,0,0,0,0,2);
      FUN_00435790(pvVar4,(int)puVar5);
      puVar5 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 2;
      FUN_004f26d0(this,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
      puVar6 = FUN_0041a400(*(void **)(param_1 + 0x2c),this,puVar6,0x20,0,0,3,0,0,
                            (undefined4 *)&DAT_00000002);
      FUN_00435790(pvVar4,(int)puVar6);
      puVar6 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 3;
      FUN_004f26d0(this,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
    }
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      local_18 = 1;
    }
  }
  ExceptionList = local_c;
  return local_18;
}

