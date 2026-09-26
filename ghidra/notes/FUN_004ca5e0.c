
void __fastcall FUN_004ca5e0(int param_1)

{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063b208;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950(&local_10,0);
  local_4 = 0;
  iVar4 = 0;
  local_14 = 0;
  local_20 = 0;
  local_1c = 0;
  local_18 = 0;
  local_24 = 0;
  *(undefined4 *)(param_1 + 0x60) = 0;
  *(undefined4 *)(param_1 + 100) = 0;
  for (iVar2 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c) + 0xd8); iVar2 != 0;
      iVar2 = *(int *)(iVar2 + 0x10)) {
    local_10 = *(uint *)(iVar2 + 100);
    if (((local_10 & 1) != 0) && ((*(uint *)(param_1 + 0x28) & 0x1000000) == 0)) {
      iVar4 = iVar4 + 1;
    }
    if (((local_10 & 0x80) != 0) && ((*(uint *)(param_1 + 0x28) & 0x2000000) == 0)) {
      local_14 = local_14 + 1;
    }
    if (((local_10 & 4) != 0) && ((*(uint *)(param_1 + 0x28) & 0x4000000) == 0)) {
      local_1c = local_1c + 1;
    }
    if (((local_10 & 8) != 0) && ((*(uint *)(param_1 + 0x28) & 0x8000000) == 0)) {
      local_20 = local_20 + 1;
    }
    if (((local_10 & 0x10) != 0) && ((*(uint *)(param_1 + 0x28) & 0x10000000) == 0)) {
      local_18 = local_18 + 1;
    }
    if ((((local_10 & 0x800) != 0) && ((*(uint *)(param_1 + 0x28) & 0x40000000) == 0)) &&
       (uVar3 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4)), (uVar3 & 0x1000000) != 0
       )) {
      local_24 = local_24 + 1;
    }
  }
  if (*(int *)(param_1 + 0x60) == 0) {
    if (local_1c < 1) {
      if ((((iVar4 < 1) || (iVar4 < local_14)) ||
          ((iVar4 < local_1c || ((iVar4 < local_18 || (iVar4 < local_20)))))) || (iVar4 < local_24))
      {
        if ((((local_14 < 1) || (local_14 < local_1c)) || (local_14 < local_18)) ||
           ((local_14 < local_20 || (local_14 < local_24)))) {
          if ((local_18 < 1) || ((local_18 < local_20 || (local_18 < local_24)))) {
            if ((local_20 < 1) || (local_20 < local_24)) {
              if (0 < local_24) {
                *(undefined4 *)(param_1 + 0x60) = 0x100400;
                *(undefined4 *)(param_1 + 100) = 0x10000;
                *(undefined4 *)(param_1 + 0x68) = 0x40000000;
              }
            }
            else {
              *(undefined4 *)(param_1 + 0x60) = 0x100102;
              *(undefined4 *)(param_1 + 100) = 0x80;
              *(undefined4 *)(param_1 + 0x68) = 0x8000000;
            }
          }
          else {
            *(undefined4 *)(param_1 + 0x60) = 0x100800;
            *(undefined4 *)(param_1 + 100) = 0x800;
            *(undefined4 *)(param_1 + 0x68) = 0x10000000;
          }
        }
        else {
          *(undefined4 *)(param_1 + 0x60) = 0x100002;
          *(undefined4 *)(param_1 + 100) = 0x400;
          *(undefined4 *)(param_1 + 0x68) = 0x2000000;
        }
      }
      else {
        *(undefined4 *)(param_1 + 0x60) = 0x100001;
        *(undefined4 *)(param_1 + 100) = 0x80000;
        *(undefined4 *)(param_1 + 0x68) = 0x1000000;
      }
    }
    else {
      *(undefined4 *)(param_1 + 0x60) = 0x100004;
      *(undefined4 *)(param_1 + 100) = 0x100;
      *(undefined4 *)(param_1 + 0x68) = 0x4000000;
    }
    if (*(int *)(param_1 + 0x60) == 0) {
      if (((*(uint *)(param_1 + 0x28) & 0x4000000) == 0) &&
         (*(int *)(*(int *)(param_1 + 0x2c) + 0x260) < *(int *)(*(int *)(param_1 + 0x2c) + 0x20c)))
      {
        *(undefined4 *)(param_1 + 0x60) = 0x100004;
        *(undefined4 *)(param_1 + 100) = 0x100;
        *(undefined4 *)(param_1 + 0x68) = 0x4000000;
      }
      if (*(int *)(param_1 + 0x60) == 0) {
        if ((((*(int *)(param_1 + 0x30) == 2) && (uVar1 = FUN_005f50e0(param_1 + 0x78), 1 < uVar1))
            && (uVar3 = FUN_005f4960((undefined4 *)(*(int *)(param_1 + 0x2c) + 4)), (uVar3 & 4) == 0
               )) && ((*(uint *)(param_1 + 0x28) & 0x20000000) == 0)) {
          *(undefined4 *)(param_1 + 0x60) = 0x80;
          *(undefined4 *)(param_1 + 100) = 0x200;
          *(undefined4 *)(param_1 + 0x68) = 0x20000000;
        }
        if (*(int *)(param_1 + 0x60) == 0) {
          if (((*(uint *)(param_1 + 0x28) & 0x1000000) == 0) &&
             (*(int *)(*(int *)(param_1 + 0x2c) + 0x268) < 1)) {
            *(undefined4 *)(param_1 + 0x60) = 0x100001;
            *(undefined4 *)(param_1 + 100) = 0x80000;
            *(undefined4 *)(param_1 + 0x68) = 0x1000000;
          }
          if (*(int *)(param_1 + 0x60) == 0) {
            if (((*(uint *)(param_1 + 0x28) & 0x2000000) == 0) &&
               (*(int *)(*(int *)(param_1 + 0x2c) + 0x26c) < 1)) {
              *(undefined4 *)(param_1 + 0x60) = 0x100002;
              *(undefined4 *)(param_1 + 100) = 0x400;
              *(undefined4 *)(param_1 + 0x68) = 0x2000000;
            }
            if (((*(int *)(param_1 + 0x60) == 0) && ((*(uint *)(param_1 + 0x28) & 0x8000000) == 0))
               && (*(int *)(*(int *)(param_1 + 0x2c) + 0x27c) <=
                   *(int *)(*(int *)(param_1 + 0x2c) + 0x16c))) {
              *(undefined4 *)(param_1 + 0x60) = 0x100102;
              *(undefined4 *)(param_1 + 100) = 0x80;
              *(undefined4 *)(param_1 + 0x68) = 0x8000000;
            }
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

