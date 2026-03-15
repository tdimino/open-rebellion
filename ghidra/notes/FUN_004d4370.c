
undefined4 __fastcall FUN_004d4370(int param_1)

{
  uint *this;
  bool bVar1;
  undefined4 *puVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int iVar6;
  undefined4 local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c400;
  local_c = ExceptionList;
  puVar4 = (undefined4 *)(param_1 + 0x50);
  local_20 = 0;
  ExceptionList = &local_c;
  puVar2 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar4,0x4000000,0,0,0,0,0,2);
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)puVar2);
  puVar2 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
  local_4 = 0;
  this = (uint *)(param_1 + 0x54);
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x40000,0,1,0x4000003,0,0,0x26,&DAT_00000002
                       );
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar2 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
  local_4 = 1;
  FUN_004f26d0(this,puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    puVar2 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar4,0x40000000,0,0,0,0,0,2);
    FUN_00435790(pvVar5,(int)puVar2);
    puVar2 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
    local_4 = 2;
    FUN_004f26d0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x40000,0,1,3,0,0,0x26,&DAT_00000002);
    FUN_00435790(pvVar5,(int)pvVar3);
    puVar2 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
    local_4 = 3;
    FUN_004f26d0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  local_1c = (undefined1 *)0x90;
  local_18 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar4,0x1000000,0,0,0,0,0,2);
    FUN_00435790(pvVar5,(int)puVar4);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
    local_4 = 4;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x40000,0,1,3,0,0,0x26,&DAT_00000002);
    FUN_00435790(pvVar5,(int)pvVar3);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
    local_4 = 5;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  local_14 = 0x90;
  local_10 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar6 = *(int *)(param_1 + 0x2c);
    local_1c = &stack0xffffffc8;
    FUN_004f26d0(&stack0xffffffc8,this);
    pvVar5 = FUN_00403d30((void *)(iVar6 + 0x2c));
    if ((pvVar5 != (void *)0x0) && (*(int *)((int)pvVar5 + 0xe0) < 0)) {
      local_20 = 1;
    }
  }
  else {
    iVar6 = FUN_004d66a0(param_1);
    if (0 < iVar6) {
      FUN_00418240(*(void **)(param_1 + 0x2c),9);
      FUN_00418240(*(void **)(param_1 + 0x2c),10);
      FUN_00418240(*(void **)(param_1 + 0x2c),0xb);
    }
  }
  ExceptionList = local_c;
  return local_20;
}

