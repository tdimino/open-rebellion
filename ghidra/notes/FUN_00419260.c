
void __thiscall FUN_00419260(void *this,uint *param_1,uint *param_2,uint *param_3)

{
  bool bVar1;
  uint uVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b198;
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
    FUN_00431170((void *)((int)this + 0x2c),param_1);
  }
  else {
    if ((uVar2 < 0x14) || (0x1b < uVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_00430830((void *)((int)this + 0x58),param_1,param_2,param_3);
    }
    else {
      if ((uVar2 < 8) || (0xf < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_0042ff70((void *)((int)this + 0x78),param_1);
      }
      else {
        if ((uVar2 < 0x30) || (0x3f < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          FUN_00403bc0((void *)((int)this + 0x8c),param_1,param_2,param_3);
        }
        else {
          if ((uVar2 < 0x40) || (0x7f < uVar2)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            FUN_0048b3d0();
          }
          else {
            if ((uVar2 < 0x80) || (0x8f < uVar2)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              FUN_00430c90((void *)((int)this + 0x44),param_1);
            }
            else {
              if ((uVar2 < 0x20) || (0x2f < uVar2)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                if ((uVar2 < 0x20) || (0x21 < uVar2)) {
                  bVar1 = false;
                }
                else {
                  bVar1 = true;
                }
                FUN_00619730();
                if (bVar1) {
                  *(uint *)((int)this + 4) = *(uint *)((int)this + 4) | 2;
                }
              }
              else {
                local_4 = 0;
                if ((uVar2 < 0x1c) || (0x1f < uVar2)) {
                  FUN_00619730();
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
  FUN_00418a60(this,param_2);
  FUN_00418a60(this,param_3);
  ExceptionList = local_c;
  return;
}

