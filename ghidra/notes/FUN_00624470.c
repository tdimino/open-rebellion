
uint FUN_00624470(void)

{
  undefined4 *puVar1;
  int iVar2;
  int *piVar3;
  uint local_8;
  int local_4;
  
  local_8 = 0xffffffff;
  FUN_0061d9d0(0x12);
  local_4 = 0;
  iVar2 = 0;
  piVar3 = &DAT_006bed20;
  do {
    puVar1 = (undefined4 *)*piVar3;
    if (puVar1 == (undefined4 *)0x0) {
      puVar1 = (undefined4 *)FUN_00619170(0x480);
      if (puVar1 != (undefined4 *)0x0) {
        DAT_006bee20 = DAT_006bee20 + 0x20;
        (&DAT_006bed20)[local_4] = puVar1;
        if (puVar1 < puVar1 + 0x120) {
          do {
            *(undefined1 *)(puVar1 + 1) = 0;
            *puVar1 = 0xffffffff;
            *(undefined1 *)((int)puVar1 + 5) = 10;
            puVar1[2] = 0;
            puVar1 = puVar1 + 9;
          } while (puVar1 < (undefined4 *)((&DAT_006bed20)[local_4] + 0x480));
        }
        local_8 = local_4 << 5;
        FUN_00624780(local_8);
      }
      break;
    }
    if (puVar1 < puVar1 + 0x120) {
      do {
        if ((*(byte *)(puVar1 + 1) & 1) == 0) {
          if (puVar1[2] == 0) {
            FUN_0061d9d0(0x11);
            if (puVar1[2] == 0) {
              InitializeCriticalSection((LPCRITICAL_SECTION)(puVar1 + 3));
              puVar1[2] = puVar1[2] + 1;
            }
            FUN_0061da50(0x11);
          }
          EnterCriticalSection((LPCRITICAL_SECTION)(puVar1 + 3));
          if ((*(byte *)(puVar1 + 1) & 1) == 0) {
            *puVar1 = 0xffffffff;
            local_8 = ((int)puVar1 - *piVar3) / 0x24 + iVar2;
            break;
          }
          LeaveCriticalSection((LPCRITICAL_SECTION)(puVar1 + 3));
        }
        puVar1 = puVar1 + 9;
      } while (puVar1 < (undefined4 *)(*piVar3 + 0x480));
    }
    if (local_8 != 0xffffffff) break;
    piVar3 = piVar3 + 1;
    local_4 = local_4 + 1;
    iVar2 = iVar2 + 0x20;
  } while ((int)piVar3 < 0x6bee20);
  FUN_0061da50(0x12);
  return local_8;
}

