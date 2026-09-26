
undefined4 __fastcall FUN_00481110(int param_1)

{
  uint *this;
  uint *this_00;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  undefined1 *local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634130;
  local_c = ExceptionList;
  bVar2 = false;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece80((undefined4 *)(param_1 + 0x30));
  local_20 = (undefined1 *)0x90;
  local_1c = 0x98;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  this = (uint *)(param_1 + 0x34);
  uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
  if ((0x8f < uVar3) && (uVar3 < 0x98)) {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x68);
    local_20 = &stack0xffffffcc;
    FUN_004f26d0(&stack0xffffffcc,this);
    pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (((pvVar4 != (void *)0x0) && ((*(byte *)((int)pvVar4 + 0x30) & 4) != 0)) &&
       ((*(uint *)((int)pvVar4 + 0x2c) & 0x200) != 0)) {
      FUN_004f26d0((undefined4 *)(param_1 + 0x30),this);
      *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
    }
  }
  if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
    this_00 = (uint *)(param_1 + 0x70);
    pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0x68) + 0x44),this_00);
    if (pvVar4 != (void *)0x0) {
      *(uint *)((int)pvVar4 + 0x28) = *(uint *)((int)pvVar4 + 0x28) & 0xdfffffff;
    }
    uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
    local_14 = 0x90;
    local_10 = 0x98;
    if ((uVar3 < 0x90) || (local_20 = (undefined1 *)0x1, 0x97 < uVar3)) {
      local_20 = (undefined1 *)0x0;
    }
    FUN_00619730();
    if (local_20 == (undefined1 *)0x0) {
      pvVar6 = FUN_0041a1d0(*(void **)(param_1 + 0x68),0,0x400,8,0,0x20000000,0x4000000,(void *)0x1)
      ;
      pvVar4 = (void *)(param_1 + 0x54);
      FUN_00435790(pvVar4,(int)pvVar6);
      puVar5 = FUN_004357b0(pvVar4,&local_20);
      local_4 = 2;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
      pvVar6 = FUN_0041a3a0(*(void **)(param_1 + 0x68),this_00,0,0x200,4,0,0,0,(void *)0x1);
      FUN_00435790(pvVar4,(int)pvVar6);
      puVar5 = FUN_004357b0(pvVar4,&local_20);
      local_4 = 3;
      FUN_004f26d0(this,puVar5);
    }
    else {
      puVar5 = FUN_00419b80(*(void **)(param_1 + 0x68),(undefined4 *)(param_1 + 0x38),0,0x400,8,0,
                            0x20000000,0x4000000,2);
      pvVar4 = (void *)(param_1 + 0x54);
      FUN_00435790(pvVar4,(int)puVar5);
      puVar5 = FUN_004357b0(pvVar4,&local_20);
      local_4 = 0;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
      pvVar6 = FUN_0041a3a0(*(void **)(param_1 + 0x68),this_00,0,0x200,4,0,0,0,(void *)0x1);
      FUN_00435790(pvVar4,(int)pvVar6);
      puVar5 = FUN_004357b0(pvVar4,&local_20);
      local_4 = 1;
      FUN_004f26d0(this,puVar5);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0((void *)(param_1 + 0x54));
    local_14 = 0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if ((bVar2) &&
       (pvVar4 = FUN_004f5940((void *)(*(int *)(param_1 + 0x68) + 0x44),this_00),
       pvVar4 != (void *)0x0)) {
      *(uint *)((int)pvVar4 + 0x28) = *(uint *)((int)pvVar4 + 0x28) | 0x20000000;
      FUN_004f26d0((void *)(param_1 + 0x30),this);
      *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
    }
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
      ExceptionList = local_c;
      return local_18;
    }
  }
  ExceptionList = local_c;
  return 1;
}

