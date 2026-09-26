
void __fastcall FUN_004e1430(int param_1)

{
  uint *this;
  int iVar1;
  undefined4 uVar2;
  bool bVar3;
  undefined4 *puVar4;
  void *pvVar5;
  void *pvVar6;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063dbc0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x50),0x40000,0,0,0,0,0,
                        2);
  pvVar6 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar6,(int)puVar4);
  puVar4 = FUN_004357b0(pvVar6,&local_18);
  this = (uint *)(param_1 + 0x54);
  local_4 = 0;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar6);
  pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x2000,0,1,3,0,0,0x15,&DAT_00000002);
  FUN_00435790(pvVar6,(int)pvVar5);
  puVar4 = FUN_004357b0(pvVar6,&local_18);
  local_4 = 1;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar6);
  local_14 = 0x90;
  local_10 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar1 = *(int *)(param_1 + 0x2c);
    local_18 = &stack0xffffffd8;
    FUN_004f26d0(&stack0xffffffd8,this);
    pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (pvVar6 != (void *)0x0) {
      uVar2 = *(undefined4 *)((int)pvVar6 + 0x9c);
      *(undefined4 *)(param_1 + 0x6c) = 0x2000;
      *(undefined4 *)(param_1 + 0x60) = uVar2;
      *(undefined4 *)(param_1 + 0x70) = 1;
      *(undefined4 *)(param_1 + 0x74) = 2;
      ExceptionList = local_c;
      return;
    }
  }
  else {
    *(uint *)(param_1 + 0x28) = *(uint *)(param_1 + 0x28) | 4;
  }
  ExceptionList = local_c;
  return;
}

