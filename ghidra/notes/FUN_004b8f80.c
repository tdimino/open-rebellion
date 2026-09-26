
void __fastcall FUN_004b8f80(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  void *pvVar6;
  undefined4 *puVar7;
  uint local_2c;
  undefined1 local_20 [4];
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006397d0;
  local_c = ExceptionList;
  local_1c = (undefined1 *)0x80;
  local_18 = 0x90;
  local_2c = 0;
  bVar3 = false;
  uVar4 = *(uint *)(*(int *)(param_1 + 0x10) + 0x34) >> 0x18;
  if ((uVar4 < 0x80) || (0x8f < uVar4)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if ((!bVar2) ||
     (pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                            (uint *)(*(int *)(param_1 + 0x10) + 0x34)), pvVar5 == (void *)0x0)) {
    ExceptionList = local_c;
    return;
  }
  iVar1 = *(int *)((int)pvVar5 + 0x34);
  if (iVar1 == 1) {
    local_2c = 4;
  }
  else if (iVar1 == 2) {
    local_2c = 5;
  }
  else if (iVar1 == 3) {
    local_2c = 6;
  }
  pvVar6 = FUN_0041a340(*(void **)(param_1 + 0xc),(undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,
                        0,0x20,0,0,0x101,local_2c,&DAT_00000002);
  pvVar5 = (void *)(param_1 + 0x18);
  FUN_00435790(pvVar5,(int)pvVar6);
  pvVar6 = FUN_0041a340(*(void **)(param_1 + 0xc),(undefined4 *)(*(int *)(param_1 + 0x10) + 0x34),0,
                        0,0x20,0,0,0x101,0x14,&DAT_00000002);
  FUN_00435790(pvVar5,(int)pvVar6);
  puVar7 = FUN_004357b0(pvVar5,local_20);
  this = (uint *)(param_1 + 0x5c);
  local_4 = 0;
  FUN_004f26d0(this,puVar7);
  do {
    local_4 = 0xffffffff;
    FUN_00619730();
    while( true ) {
      local_14 = 0x90;
      local_10 = 0x98;
      if (((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) || (bVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) {
        FUN_005f58b0(pvVar5);
        ExceptionList = local_c;
        return;
      }
      iVar1 = *(int *)(param_1 + 0xc);
      local_1c = &stack0xffffffc0;
      FUN_004f26d0(&stack0xffffffc0,this);
      pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar6 == (void *)0x0) || (*(int *)((int)pvVar6 + local_2c * 4 + 0x48) < 1)) break;
      bVar3 = true;
    }
    puVar7 = FUN_004357b0(pvVar5,local_20);
    local_4 = 1;
    FUN_004f26d0(this,puVar7);
  } while( true );
}

