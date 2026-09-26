
int * FUN_00624320(int param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  uint dwBytes;
  int *piVar3;
  int *piVar4;
  
  dwBytes = param_2 * param_1;
  if (dwBytes < 0xffffffe1) {
    if (dwBytes == 0) {
      dwBytes = 0x10;
    }
    else {
      dwBytes = dwBytes + 0xf & 0xfffffff0;
    }
  }
  do {
    piVar3 = (int *)0x0;
    if (dwBytes < 0xffffffe1) {
      if (DAT_006af194 < dwBytes) {
LAB_00624394:
        if (piVar3 != (int *)0x0) {
          return piVar3;
        }
      }
      else {
        FUN_0061d9d0(9);
        piVar3 = FUN_0061deb0(dwBytes >> 4);
        FUN_0061da50(9);
        if (piVar3 != (int *)0x0) {
          piVar4 = piVar3;
          for (uVar2 = dwBytes >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
            *piVar4 = 0;
            piVar4 = piVar4 + 1;
          }
          for (uVar2 = dwBytes & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
            *(undefined1 *)piVar4 = 0;
            piVar4 = (int *)((int)piVar4 + 1);
          }
          goto LAB_00624394;
        }
      }
      piVar3 = HeapAlloc(DAT_006bed14,8,dwBytes);
    }
    if ((piVar3 != (int *)0x0) || (DAT_006be82c == 0)) {
      return piVar3;
    }
    iVar1 = FUN_0061f060(dwBytes);
    if (iVar1 == 0) {
      return (int *)0x0;
    }
  } while( true );
}

