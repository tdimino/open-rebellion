
void __thiscall FUN_0049d3b0(void *this,uint *param_1,undefined4 *param_2,undefined4 *param_3)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637108;
  local_c = ExceptionList;
  uVar2 = *param_1 >> 0x18;
  if ((uVar2 < 0x90) || (0x97 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    FUN_004c5d20((void *)((int)this + 8),param_1);
  }
  else {
    if ((uVar2 < 0x14) || (0x1b < uVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      if ((uVar2 < 8) || (0xf < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        if ((uVar2 < 0x30) || (0x3f < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          if ((uVar2 < 0x40) || (0x7f < uVar2)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (!bVar1) {
            if ((uVar2 < 0x80) || (0x8f < uVar2)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              FUN_004c5850((void *)((int)this + 0x20),param_1);
            }
            else {
              if ((uVar2 < 0x28) || (0x2f < uVar2)) {
                iVar3 = 0;
              }
              else {
                iVar3 = 1;
              }
              FUN_00619730();
              if (iVar3 == 0) {
                if ((uVar2 < 0x1c) || (0x1f < uVar2)) {
                  if ((uVar2 < 0x10) || (0x13 < uVar2)) {
                    bVar1 = false;
                  }
                  else {
                    bVar1 = true;
                  }
                  local_4 = iVar3;
                  FUN_00619730();
                  if (!bVar1) {
                    FUN_00619730();
                  }
                }
                local_4 = 0xffffffff;
                FUN_00619730();
              }
            }
          }
        }
      }
    }
  }
  FUN_0049cdb0(param_2);
  FUN_0049cdb0(param_3);
  ExceptionList = local_c;
  return;
}

