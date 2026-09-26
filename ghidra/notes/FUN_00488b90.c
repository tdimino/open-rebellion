
void __fastcall FUN_00488b90(int param_1)

{
  uint *puVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  
  iVar3 = thunk_FUN_005f5060(param_1 + 0x28);
  if (iVar3 != 0) {
    if ((*(uint *)(param_1 + 8) & 0x20000000) == 0) {
      iVar3 = thunk_FUN_005f5060(param_1 + 0x28);
      if (iVar3 != 0) {
        FUN_0041d850(iVar3);
        *(uint *)(param_1 + 8) = *(uint *)(param_1 + 8) | 0x20000000;
      }
    }
    *(uint *)(param_1 + 8) = *(uint *)(param_1 + 8) & 0xfffffff7;
    return;
  }
  puVar1 = (uint *)(param_1 + 8);
  iVar3 = FUN_005f4960(puVar1);
  if (iVar3 != 0) {
    uVar2 = *puVar1;
    if ((uVar2 & 1) == 0) {
      if ((uVar2 & 2) == 0) {
        if ((uVar2 & 4) != 0) {
          *puVar1 = *puVar1 & 0xfffffffb;
        }
      }
      else if (*(int *)(param_1 + 0x40) != 0) {
        iVar3 = FUN_0041d6d0();
        if (iVar3 != 0) {
          piVar4 = (int *)thunk_FUN_005f5060(param_1 + 0x1c);
          if (piVar4 != (int *)0x0) {
            iVar3 = (**(code **)(*piVar4 + 0xc))();
            if (iVar3 == 2) {
              FUN_0041d710((LPARAM)piVar4);
              *(undefined4 *)(param_1 + 0x40) = 0;
              *puVar1 = *puVar1 & 0xfffffffd;
              return;
            }
          }
        }
      }
    }
    else if (*(int *)(param_1 + 0x40) != 0) {
      iVar3 = FUN_0041d6d0();
      if (iVar3 != 0) {
        piVar4 = (int *)thunk_FUN_005f5060(param_1 + 0x1c);
        if (piVar4 != (int *)0x0) {
          iVar3 = (**(code **)(*piVar4 + 0xc))();
          if (iVar3 == 1) {
            FUN_0041d730(piVar4);
            *(undefined4 *)(param_1 + 0x40) = 0;
            *puVar1 = *puVar1 & 0xfffffffe;
            return;
          }
        }
      }
    }
  }
  return;
}

