
void __thiscall FUN_00558880(void *this,void *param_1)

{
  uint uVar1;
  void *local_4;
  
  local_4 = this;
  FUN_00558820(&local_4);
  local_4 = param_1;
  if (((((((uint)param_1 & 3) != 0) && (((uint)param_1 & 3) < 4)) &&
       (uVar1 = (uint)param_1 >> 2 & 3, uVar1 != 0)) &&
      ((uVar1 < 4 &&
       (((uVar1 = (uint)param_1 >> 4 & 3, uVar1 != 0 && (uVar1 < 4)) ||
        (((uint)param_1 & 0x30) == 0)))))) && (((uint)param_1 & 0xffffffc0) == 0)) {
    *(void **)this = param_1;
  }
  thunk_FUN_00619730();
  return;
}

