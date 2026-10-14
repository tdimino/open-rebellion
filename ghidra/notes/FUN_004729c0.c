
void __thiscall FUN_004729c0(int *param_1,int *param_2,undefined4 *param_3)

{
  int iVar1;
  
  if ((int *)param_1[7] != (int *)0x0) {
    iVar1 = (**(code **)(*(int *)param_1[7] + 0x18))(param_2,param_3);
    if (iVar1 != 0) {
      (**(code **)(*(int *)param_1[7] + 0x20))();
      if ((undefined4 *)param_1[7] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)param_1[7])(1);
      }
      param_1[7] = 0;
    }
  }
  if (param_1[7] == 0) {
    iVar1 = FUN_005f5e90((int)(param_1 + 0xc));
    param_1[7] = iVar1;
    if ((iVar1 == 0) && (param_1[0x10] != 0)) {
      (**(code **)(*param_1 + 8))
                (param_1[0x11],param_1 + 0x12,param_1[0x13],param_1[0x14],param_1[0x15],1);
      iVar1 = FUN_005f5e90((int)(param_1 + 0xc));
      param_1[7] = iVar1;
    }
    if ((int *)param_1[7] != (int *)0x0) {
      (**(code **)(*(int *)param_1[7] + 0x1c))();
      if (*(int **)(param_1[7] + 0xc) != (int *)0x0) {
        (**(code **)(**(int **)(param_1[7] + 0xc) + 0x20))();
      }
      if (((int *)param_1[8] != (int *)0x0) && (*(int *)param_1[8] != 0)) {
        *param_3 = 1;
        *param_2 = *(int *)(*(int *)param_1[8] + 0x1c);
        if (*(int **)(*(int *)param_1[8] + 0x2c) == (int *)0x0) {
          iVar1 = 0;
        }
        else {
          iVar1 = FUN_005fc0e0(*(int **)(*(int *)param_1[8] + 0x2c));
        }
        param_2[2] = *param_2 + iVar1;
        param_2[1] = *(int *)(*(int *)param_1[8] + 0x20);
        if (*(int **)(*(int *)param_1[8] + 0x2c) != (int *)0x0) {
          iVar1 = FUN_005fc0f0(*(int **)(*(int *)param_1[8] + 0x2c));
          param_2[1] = iVar1;
          param_2[3] = iVar1;
          return;
        }
        param_2[1] = 0;
        param_2[3] = 0;
      }
    }
  }
  return;
}

