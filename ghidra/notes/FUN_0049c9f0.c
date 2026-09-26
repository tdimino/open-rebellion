
void __thiscall FUN_0049c9f0(void *this,void *param_1)

{
  undefined4 *puVar1;
  void *unaff_EBX;
  int iVar2;
  uint unaff_EBP;
  int iVar3;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637090;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_005f4db0(param_1,this);
  FUN_005f4db0(param_1,(int)this + 0x78);
  FUN_005f4db0(param_1,(int)this + 0x7c);
  FUN_005f49a0((void *)((int)this + 4),param_1);
  (**(code **)(*(int *)((int)this + 8) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x20) + 0x18))(param_1);
  iVar3 = (int)this + 0x80;
  iVar2 = 0x1e;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  iVar3 = (int)this + 0xf8;
  iVar2 = 1;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  if (*(void **)((int)this + 0x74) == (void *)0x0) {
    local_4 = 0;
    FUN_005f4db0(param_1,&local_4);
  }
  else {
    puVar1 = FUN_00403040(*(void **)((int)this + 0x74),&local_4);
    pvStack_c._0_1_ = 1;
    FUN_004f26d0(&stack0xffffffe8,puVar1);
    pvStack_c = (void *)((uint)pvStack_c._1_3_ << 8);
    FUN_00619730();
    local_4 = unaff_EBP >> 0x18;
    FUN_005f4db0(param_1,&local_4);
    FUN_004ecea0(&stack0xffffffe8,param_1);
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EBX;
  return;
}

