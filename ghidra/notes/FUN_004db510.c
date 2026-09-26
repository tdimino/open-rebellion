
int __fastcall FUN_004db510(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int local_28;
  int local_24;
  undefined1 local_20 [4];
  undefined4 local_1c;
  undefined4 local_18;
  void *local_14;
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d18b;
  local_c = ExceptionList;
  local_28 = 0;
  local_24 = 1000;
  this = (uint *)(param_1 + 0x40);
  ExceptionList = &local_c;
  FUN_004ece80(this);
  local_14 = (void *)(param_1 + 0x58);
  *(undefined4 *)(param_1 + 0x50) = 0;
  for (pvVar3 = (void *)thunk_FUN_005f5060((int)local_14); pvVar3 != (void *)0x0;
      pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
    puVar4 = FUN_00403040(pvVar3,local_20);
    iVar1 = *(int *)(param_1 + 0x2c);
    local_10 = &stack0xffffffc4;
    local_4 = 0;
    FUN_004f26d0(&stack0xffffffc4,puVar4);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) &&
        ((*(byte *)((int)pvVar5 + 0x28) & 3) == 0)) &&
       ((0 < *(int *)((int)pvVar5 + 0x114) && (*(int *)((int)pvVar5 + 0x60) < local_24)))) {
      *(int *)(param_1 + 0x50) = *(int *)((int)pvVar5 + 0x114);
      local_24 = *(int *)((int)pvVar5 + 0x60);
      puVar4 = FUN_00403040(pvVar5,&local_1c);
      local_4 = 1;
      FUN_004f26d0(this,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      local_28 = 1;
    }
  }
  if (local_28 == 0) {
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x80,0,0,0,0,0,4,&DAT_00000002);
    pvVar3 = (void *)(param_1 + 0x68);
    FUN_00435790(pvVar3,(int)pvVar5);
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x80,0,0,0,0,0,0x15,(void *)0x1);
    FUN_00435790(pvVar3,(int)pvVar5);
    pvVar5 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x80,0,0,0,0,0,8,(void *)0x1);
    FUN_00435790(pvVar3,(int)pvVar5);
    puVar4 = FUN_004357b0(pvVar3,&local_1c);
    local_4 = 2;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar3);
    pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3800003,0,0x40000000,6,(void *)0x1
                         );
    FUN_00435790(pvVar3,(int)pvVar5);
    pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3800003,0,0x40000000,0,(void *)0x1
                         );
    FUN_00435790(pvVar3,(int)pvVar5);
    pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3800003,0,0x40000000,0x33,
                          (void *)0x1);
    FUN_00435790(pvVar3,(int)pvVar5);
    puVar4 = FUN_004357b0(pvVar3,&local_1c);
    local_4 = 3;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar3);
    local_1c = 0x90;
    local_18 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x2c);
      local_10 = &stack0xffffffc4;
      FUN_004f26d0(&stack0xffffffc4,this);
      pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar3 != (void *)0x0) {
        local_10 = (undefined1 *)FUN_00618b70(0x1c);
        local_4 = 4;
        if (local_10 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004ecc70(local_10,this);
        }
        local_4 = 0xffffffff;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_14,pvVar5);
          *(uint *)((int)pvVar3 + 0x28) = *(uint *)((int)pvVar3 + 0x28) | 0x800000;
          FUN_00433e30((int)pvVar3);
          local_28 = 1;
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_28;
}

