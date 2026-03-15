
uint __fastcall FUN_0054a030(int *param_1)

{
  uint uVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  uint local_1c;
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647d48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_004f9700(param_1);
  if (uVar1 != 0) {
    this = (void *)FUN_00505cd0();
    do {
      if (this == (void *)0x0) {
        ExceptionList = local_c;
        return uVar1;
      }
      FUN_004f26d0(&local_1c,(undefined4 *)((int)this + 0x68));
      local_4 = 1;
      puVar2 = FUN_004ece40(&local_1c);
      if (puVar2 != (uint *)0x0) {
        iVar3 = FUN_00505970(&local_1c);
        uVar1 = (uint)(iVar3 != 0);
        if (uVar1 != 0) {
          puVar2 = FUN_004025b0(this,local_18);
          local_4._0_1_ = 2;
          uVar1 = FUN_00520c30(puVar2);
          local_4 = CONCAT31(local_4._1_3_,1);
          FUN_00619730();
        }
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      local_18[1] = 0x30;
      local_18[2] = 0x40;
      local_4 = 0;
      this = (void *)FUN_004f6010(this,local_18 + 1,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    } while (uVar1 != 0);
  }
  ExceptionList = local_c;
  return uVar1;
}

