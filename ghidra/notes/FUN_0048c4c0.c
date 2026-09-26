
void __thiscall FUN_0048c4c0(void *this,uint param_1,int param_2)

{
  uint *this_00;
  bool bVar1;
  ushort uVar2;
  int *piVar3;
  int *piVar4;
  uint uVar5;
  int iVar6;
  uint local_30;
  uint local_2c;
  uint local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00635388;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_30);
  iVar6 = param_2;
  local_4 = 0;
  if (param_1 == 0x200) {
    FUN_004f26d0(&local_30,(undefined4 *)(param_2 + 0x20));
    FUN_004f26d0(&param_1,(undefined4 *)(iVar6 + 0x2c));
    local_4 = CONCAT31(local_4._1_3_,1);
    piVar3 = FUN_004f2d10(*(int *)((int)this + 0x20),&local_30);
    if (*(int *)((int)this + 100) == 0) {
      *(undefined4 *)((int)this + 100) = 1;
      *(ushort *)((int)this + 0x2e) = (*(int *)((int)this + 0x20) != 1) + 0x412;
      *(ushort *)((int)this + 0x30) = (*(int *)((int)this + 0x20) != 1) + 0x47c;
      FUN_004f26d0((void *)((int)this + 0x68),&local_30);
    }
    if (*(int *)((int)this + 0x20) == 2) {
      *(undefined4 *)((int)this + 0x40) = 0;
    }
    else {
      local_24 = 0x30;
      local_20 = 0x3c;
      if ((local_30 >> 0x18 < 0x30) || (0x3b < local_30 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (*(int *)(iVar6 + 0x28) == 0) {
          *(undefined4 *)((int)this + 0x40) = 0;
        }
        else {
          local_1c = 100;
          local_18 = 0x65;
          if ((param_1 >> 0x18 < 100) || (100 < param_1 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          iVar6 = param_2;
          if (bVar1) {
            *(undefined4 *)((int)this + 0x40) = 1;
          }
        }
      }
      else {
        *(undefined4 *)((int)this + 0x40) = 0;
      }
    }
    if ((*(int *)((int)this + 0x40) != 0) && (*(int *)(iVar6 + 0x30) != 0)) {
      this_00 = (uint *)((int)this + 0x68);
      FUN_004f26d0(this_00,&local_30);
      FUN_004f26d0((void *)((int)this + 0x5c),&local_30);
      local_14 = 0x30;
      local_10 = 0x38;
      if ((*this_00 >> 0x18 < 0x30) || (0x37 < *this_00 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        switch(*this_00 & 0xffffff) {
        case 0x240:
          *(undefined4 *)((int)this + 0x28) = 0x17;
          break;
        case 0x241:
          *(undefined4 *)((int)this + 0x28) = 0x18;
          uVar2 = FUN_004c4990(&DAT_000000e5);
          *(ushort *)((int)this + 0x32) = uVar2;
          break;
        case 0x242:
          *(undefined4 *)((int)this + 0x28) = 0x16;
          if ((*(byte *)(piVar3 + 0x2c) & 8) == 0) {
            uVar2 = FUN_004c4990(&DAT_000000d1);
            *(ushort *)((int)this + 0x32) = uVar2;
          }
          else {
            uVar2 = FUN_004c4990(&DAT_000000db);
            *(ushort *)((int)this + 0x32) = uVar2;
          }
          break;
        case 0x243:
          *(undefined4 *)((int)this + 0x28) = 0x19;
          break;
        case 0x280:
          *(undefined4 *)((int)this + 0x28) = 0x1a;
          break;
        case 0x281:
          *(undefined4 *)((int)this + 0x28) = 0x1b;
        }
      }
      else {
        *(undefined4 *)((int)this + 0x28) = 0x15;
      }
      if (piVar3 == (int *)0x0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x44),0x71ea,piVar4,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (piVar3 == (int *)0x0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)((int)this + 0x50),0x71eb,piVar4,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      local_2c = 0x30;
      local_28 = 0x3c;
      local_4._0_1_ = 2;
      uVar5 = (**(code **)(*piVar3 + 4))();
      if ((uVar5 < local_2c) || (local_28 <= uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      if (bVar1) {
        uVar5 = FUN_004c5000(piVar3);
        *(short *)((int)this + 0x2c) = (short)uVar5;
      }
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

