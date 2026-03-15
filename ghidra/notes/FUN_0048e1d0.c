
void __fastcall FUN_0048e1d0(int param_1)

{
  ushort uVar1;
  int *piVar2;
  int *this;
  uint uVar3;
  uint *puVar4;
  int *piVar5;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635730;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004f2ec0(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x68));
  this = FUN_004f2ec0(*(int *)(param_1 + 0x20),(uint *)(param_1 + 0x6c));
  piVar5 = piVar2;
  if (*(int *)(param_1 + 0x20) == 1) {
    piVar5 = this;
    this = piVar2;
  }
  if ((this == (int *)0x0) || (piVar5 == (int *)0x0)) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  else {
    *(ushort *)(param_1 + 0x2e) = (*(int *)(param_1 + 0x20) != 1) + 0x412;
    uVar3 = FUN_004c5000(this);
    *(short *)(param_1 + 0x2c) = (short)uVar3;
    *(ushort *)(param_1 + 0x30) = (*(int *)(param_1 + 0x20) != 1) + 0x47c;
    FUN_0060b9d0((void *)(param_1 + 0x44),0x71b0,this + 0xc,piVar5 + 0xc,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    FUN_0060b9d0((void *)(param_1 + 0x50),0x71b1,this + 0xc,piVar5 + 0xc,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    local_4 = 0;
    puVar4 = FUN_004025b0(this,&local_10);
    uVar3 = *puVar4;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar3 == 0x32000242) {
      if ((*(byte *)(this + 0x2c) & 8) != 0) {
        uVar1 = FUN_004c4990(&DAT_000000da);
        *(ushort *)(param_1 + 0x32) = uVar1;
        ExceptionList = local_c;
        return;
      }
      uVar1 = FUN_004c4990(&DAT_000000d0);
      *(ushort *)(param_1 + 0x32) = uVar1;
      ExceptionList = local_c;
      return;
    }
    local_4 = 1;
    puVar4 = FUN_004025b0(this,&local_10);
    uVar3 = *puVar4;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (uVar3 == 0x31000241) {
      uVar1 = FUN_004c4990(&DAT_000000e4);
      *(ushort *)(param_1 + 0x32) = uVar1;
      ExceptionList = local_c;
      return;
    }
  }
  ExceptionList = local_c;
  return;
}

