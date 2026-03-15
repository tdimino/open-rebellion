
void __fastcall FUN_005fd170(uint *param_1)

{
  char *pcVar1;
  undefined1 *puVar2;
  uint uVar3;
  
  uVar3 = FUN_005fc100(param_1);
  if (uVar3 == 8) {
    if ((*param_1 != 0) && (pcVar1 = (char *)param_1[1], pcVar1 != (char *)0x0)) {
      if (*(int *)(*param_1 + 0x10) != 1) {
        *(char *)(param_1 + 2) = *pcVar1;
        return;
      }
      *(undefined1 *)(param_1 + 2) = 0;
      if (*pcVar1 != '\0') {
        *(char *)(param_1 + 2) = pcVar1[1];
        return;
      }
      if (2 < (byte)pcVar1[1]) {
        *(char *)(param_1 + 2) = pcVar1[2];
        return;
      }
    }
  }
  else {
    uVar3 = FUN_005fc100(param_1);
    if (uVar3 == 0x18) {
      puVar2 = (undefined1 *)param_1[1];
      *(undefined1 *)((int)param_1 + 0xb) = *puVar2;
      *(undefined1 *)((int)param_1 + 10) = puVar2[1];
      *(undefined1 *)((int)param_1 + 9) = puVar2[2];
    }
  }
  return;
}

