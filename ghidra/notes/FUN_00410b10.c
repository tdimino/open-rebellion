
void __fastcall FUN_00410b10(int *param_1)

{
  bool bVar1;
  int iVar2;
  
  param_1[0x46] = 1;
  FUN_0040f850((int)param_1);
  KillTimer((HWND)param_1[6],0x191);
  FUN_00597340();
  FUN_00597360();
  FUN_00597340();
  FUN_00597050();
  (**(code **)(*param_1 + 0x38))();
  param_1[0x4b] = 0x2774;
  FUN_006075e0(param_1,0x2774);
  if ((((((int *)param_1[0x4f] == (int *)0x0) || (param_1[0x50] == 0)) || (param_1[0x51] == 0)) ||
      (((param_1[0x52] == 0 || (param_1[0x53] == 0)) ||
       ((param_1[0x54] == 0 || ((param_1[0x55] == 0 || (param_1[0x56] == 0)))))))) ||
     ((param_1[0x57] == 0 ||
      ((((param_1[0x58] == 0 || (param_1[0x59] == 0)) || (param_1[0x5a] == 0)) ||
       (param_1[0x5b] == 0)))))) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (bVar1) {
    (**(code **)(*(int *)param_1[0x4f] + 0x10))();
    (**(code **)(*(int *)param_1[0x50] + 4))(5);
    (**(code **)(*(int *)param_1[0x52] + 4))(5);
    (**(code **)(*(int *)param_1[0x53] + 4))(5);
    if ((int *)param_1[0x61] != (int *)0x0) {
      (**(code **)(*(int *)param_1[0x61] + 4))(5);
    }
    iVar2 = FUN_00609eb0(param_1[0x61]);
    if (iVar2 == 0) {
      (**(code **)(*(int *)param_1[0x50] + 0x10))();
    }
  }
  return;
}

