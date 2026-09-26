
void __thiscall FUN_004c1550(void *this,undefined4 param_1,int *param_2)

{
  bool bVar1;
  
  bVar1 = true;
  if ((*param_2 != 0x90) || (param_2[1] != 1)) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)((int)this + 0x168) + 0x30) = DAT_006b28cc + 10;
    return;
  }
  if ((*param_2 == 0x90) && (param_2[1] == 2)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)((int)this + 0x168) + 0x34) = DAT_006b28cc + 10;
    return;
  }
  if ((*param_2 == 0x90) && (param_2[1] == 4)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)((int)this + 0x168) + 0x58) = DAT_006b28cc + 10;
    return;
  }
  if ((*param_2 == 0x90) && (param_2[1] == 3)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)((int)this + 0x168) + 0x2c) = DAT_006b28cc + 10;
    return;
  }
  thunk_FUN_004c2230((int)this);
  return;
}

