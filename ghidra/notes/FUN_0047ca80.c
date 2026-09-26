
undefined4 __fastcall FUN_0047ca80(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006339b0;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  FUN_004ece80((undefined4 *)(param_1 + 0x30));
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  if ((*(byte *)(param_1 + 0x20) & 1) == 0) {
    FUN_004ece80((undefined4 *)(param_1 + 0x34));
  }
  else {
    this = (uint *)(param_1 + 0x34);
    uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((0x8f < uVar3) && (uVar3 < 0x98)) {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_14 = &stack0xffffffd4;
      FUN_004f26d0(&stack0xffffffd4,this);
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar4 != (void *)0x0) && ((*(byte *)((int)pvVar4 + 0x2c) & 0x10) != 0)) {
        FUN_004f26d0((undefined4 *)(param_1 + 0x30),this);
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
      }
    }
    if ((*(byte *)(param_1 + 0x20) & 0x10) != 0) {
      ExceptionList = local_c;
      return 1;
    }
    uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((uVar3 < 0x90) || (0x97 < uVar3)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      puVar5 = FUN_00419b80(*(void **)(param_1 + 0x68),(undefined4 *)(param_1 + 0x38),0,4,0,0,0,0,2)
      ;
      pvVar4 = (void *)(param_1 + 0x54);
      FUN_00435790(pvVar4,(int)puVar5);
      puVar5 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 0;
      FUN_004f26d0(this,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
      puVar5 = FUN_0041a400(*(void **)(param_1 + 0x68),this,(undefined4 *)(param_1 + 0x38),0,0x10,0,
                            0,0,0,(undefined4 *)&DAT_00000002);
      FUN_00435790(pvVar4,(int)puVar5);
      puVar5 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 1;
      FUN_004f26d0(this,puVar5);
    }
    else {
      pvVar6 = FUN_00419a00(*(void **)(param_1 + 0x68),0,4,0,0,0,0,0x3b,(void *)0x1);
      pvVar4 = (void *)(param_1 + 0x54);
      FUN_00435790(pvVar4,(int)pvVar6);
      puVar5 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 2;
      FUN_004f26d0(this,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
      pvVar6 = FUN_0041a340(*(void **)(param_1 + 0x68),this,0,0x10,0,0,0,0,0,(void *)0x1);
      FUN_00435790(pvVar4,(int)pvVar6);
      puVar5 = FUN_004357b0(pvVar4,&local_14);
      local_4 = 3;
      FUN_004f26d0(this,puVar5);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0((void *)(param_1 + 0x54));
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
      FUN_004f26d0((void *)(param_1 + 0x30),this);
      *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
    }
  }
  if ((*(byte *)(param_1 + 0x20) & 0x10) != 0) {
    ExceptionList = local_c;
    return 1;
  }
  ExceptionList = local_c;
  return 0;
}

