
int __fastcall FUN_004dfcf0(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  short sVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  void *pvVar7;
  int local_24;
  int local_20;
  undefined1 local_1c [4];
  undefined4 local_18;
  undefined4 local_14;
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d8bb;
  local_c = ExceptionList;
  local_24 = 0;
  local_20 = 1000;
  this = (uint *)(param_1 + 0x58);
  ExceptionList = &local_c;
  FUN_004ece80(this);
  *(undefined4 *)(param_1 + 0x68) = 0;
  for (pvVar4 = (void *)thunk_FUN_005f5060(param_1 + 0x7c); pvVar4 != (void *)0x0;
      pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
    puVar5 = (undefined4 *)FUN_00403040(pvVar4,local_1c);
    iVar1 = *(int *)(param_1 + 0x2c);
    local_10 = &stack0xffffffc8;
    local_4 = 0;
    FUN_004f26d0(&stack0xffffffc8,puVar5);
    pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((((pvVar6 != (void *)0x0) && ((*(byte *)((int)pvVar6 + 0x30) & 1) != 0)) &&
        ((*(byte *)((int)pvVar6 + 0x28) & 3) == 0)) &&
       ((0 < *(int *)((int)pvVar6 + 0x114) && (*(int *)((int)pvVar6 + 0x5c) < local_20)))) {
      *(int *)(param_1 + 0x68) = *(int *)((int)pvVar6 + 0x114);
      local_20 = *(int *)((int)pvVar6 + 0x5c);
      puVar5 = (undefined4 *)FUN_00403040(pvVar6,&local_18);
      local_4 = 1;
      FUN_004f26d0(this,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      local_24 = 1;
    }
  }
  if (local_24 == 0) {
    pvVar6 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x40,0,0,0,0,0,0x13,(void *)0x1);
    pvVar4 = (void *)(param_1 + 0x40);
    FUN_00435790(pvVar4,(int)pvVar6);
    pvVar6 = (void *)(param_1 + 0x7c);
    sVar3 = FUN_005f50e0((int)pvVar6);
    if (sVar3 != 0) {
      pvVar7 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x40,0,0,0,0,0,4,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar7);
    }
    puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_18);
    local_4 = 2;
    FUN_004f26d0(this,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    pvVar7 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x400003,0,0x40000000,5,(void *)0x1)
    ;
    FUN_00435790(pvVar4,(int)pvVar7);
    sVar3 = FUN_005f50e0((int)pvVar6);
    if (sVar3 != 0) {
      pvVar7 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x400003,0,0x40000000,0x33,
                            (void *)0x1);
      FUN_00435790(pvVar4,(int)pvVar7);
    }
    puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_18);
    local_4 = 3;
    FUN_004f26d0(this,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    local_18 = 0x90;
    local_14 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x2c);
      local_10 = &stack0xffffffc8;
      local_24 = 1;
      FUN_004f26d0(&stack0xffffffc8,this);
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar4 != (void *)0x0) {
        local_10 = (undefined1 *)FUN_00618b70(0x1c);
        local_4 = 4;
        if (local_10 == (undefined1 *)0x0) {
          pvVar7 = (void *)0x0;
        }
        else {
          pvVar7 = FUN_004ecc70(local_10,this);
        }
        local_4 = 0xffffffff;
        if (pvVar7 != (void *)0x0) {
          FUN_004f57b0(pvVar6,pvVar7);
          *(uint *)((int)pvVar4 + 0x28) = *(uint *)((int)pvVar4 + 0x28) | 0x400000;
          FUN_00433e30((int)pvVar4);
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_24;
}

