
void __thiscall FUN_00458d20(int param_1,uint *param_2)

{
  bool bVar1;
  undefined4 uStack_10;
  
  uStack_10 = 0;
  if ((*param_2 >> 0x18 < 0x2c) || (0x2c < *param_2 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    uStack_10 = 0x6c;
  }
  else {
    if ((*param_2 >> 0x18 < 0x2d) || (0x2d < *param_2 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      uStack_10 = 0x6b;
    }
    else {
      if ((*param_2 >> 0x18 < 0x28) || (0x28 < *param_2 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        uStack_10 = 0x68;
      }
      else {
        if ((*param_2 >> 0x18 < 0x29) || (0x29 < *param_2 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          uStack_10 = 0x69;
        }
        else {
          if ((*param_2 >> 0x18 < 0x2a) || (0x2a < *param_2 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            uStack_10 = 0x6a;
          }
        }
      }
    }
  }
  if ((uStack_10 != 0) && (uStack_10 != *(int *)(param_1 + 0x160))) {
    FUN_0060d7e0(uStack_10,1);
  }
  return;
}

