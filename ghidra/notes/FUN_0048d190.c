
void __thiscall FUN_0048d190(void *this,int param_1,int param_2)

{
  bool bVar1;
  ushort uVar2;
  void *this_00;
  uint *puVar3;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635518;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_004ece30(&local_1c);
  local_4._0_1_ = 1;
  if ((param_1 == 0x221) && (*(int *)((int)this + 100) == 0)) {
    *(undefined4 *)((int)this + 100) = 1;
    if (*(int *)((int)this + 0x20) == 1) {
      this_00 = (void *)FUN_004f3e50(1);
      puVar3 = FUN_0042d170(this_00,&param_1);
      local_14 = 0x71;
      local_10 = 0x72;
      local_4._0_1_ = 2;
      if ((*puVar3 >> 0x18 < 0x71) || (0x71 < *puVar3 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 1;
      FUN_00619730();
      if (bVar1) {
        *(undefined4 *)((int)this + 0x40) = 1;
      }
      if (*(int *)(param_2 + 0x28) == 0) {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    if (*(int *)((int)this + 0x40) != 0) {
      *(undefined2 *)((int)this + 0x2e) = 0x421;
      *(undefined4 *)((int)this + 0x28) = 0x16;
      *(undefined2 *)((int)this + 0x30) = 0x46c;
      uVar2 = FUN_004c4990(&DAT_0000011d);
      *(ushort *)((int)this + 0x32) = uVar2;
      FUN_0060b9d0((void *)((int)this + 0x44),0x71b8,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
      FUN_0060b9d0((void *)((int)this + 0x50),0x71b9,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0,(undefined4 *)0x0);
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

