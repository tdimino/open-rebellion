
void * __thiscall FUN_006162b0(void *this,char *param_1,char *param_2)

{
  char cVar1;
  byte bVar2;
  int *piVar3;
  bool bVar4;
  char *pcVar5;
  char *pcVar6;
  int iVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  char *pcVar11;
  
  pcVar5 = param_1;
  uVar8 = 0xffffffff;
  do {
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    cVar1 = *param_1;
    param_1 = param_1 + 1;
  } while (cVar1 != '\0');
  iVar9 = ~uVar8 - 1;
  uVar8 = 0xffffffff;
  pcVar11 = param_2;
  do {
    if (uVar8 == 0) break;
    uVar8 = uVar8 - 1;
    cVar1 = *pcVar11;
    pcVar11 = pcVar11 + 1;
  } while (cVar1 != '\0');
  iVar10 = ~uVar8 - 1;
  uVar8 = *(uint *)(*(int *)(*(int *)this + 4) + 0x30 + (int)this);
  if ((uint)(iVar9 + iVar10) < uVar8) {
    param_1 = (char *)((uVar8 - iVar10) - iVar9);
  }
  else {
    param_1 = (char *)0x0;
  }
  if (((*(byte *)((int)this + *(int *)(*(int *)this + 4) + 0x24) & 10) == 0) &&
     (pcVar6 = param_1 + -1, bVar4 = 0 < (int)param_1, pcVar11 = param_1, param_1 = pcVar6, bVar4))
  {
    do {
      bVar2 = *(byte *)((int)this + *(int *)(*(int *)this + 4) + 0x2c);
      piVar3 = *(int **)((int)this + *(int *)(*(int *)this + 4) + 4);
      if ((byte *)piVar3[7] < (byte *)piVar3[8]) {
        *(byte *)piVar3[7] = bVar2;
        uVar8 = (uint)bVar2;
        piVar3[7] = piVar3[7] + 1;
      }
      else {
        uVar8 = (**(code **)(*piVar3 + 0x1c))(bVar2);
      }
      if (uVar8 == 0xffffffff) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
      }
      pcVar11 = pcVar11 + -1;
      param_1 = param_1 + -1;
    } while (pcVar11 != (char *)0x0);
  }
  if ((iVar9 != 0) &&
     (iVar7 = (**(code **)(**(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this) + 0x14))
                        (pcVar5,iVar9), iVar7 != iVar9)) {
    *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
         *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
  }
  if (((*(byte *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this) & 8) != 0) && (0 < (int)param_1)) {
    do {
      bVar2 = *(byte *)((int)this + *(int *)(*(int *)this + 4) + 0x2c);
      piVar3 = *(int **)((int)this + *(int *)(*(int *)this + 4) + 4);
      if ((byte *)piVar3[7] < (byte *)piVar3[8]) {
        *(byte *)piVar3[7] = bVar2;
        uVar8 = (uint)bVar2;
        piVar3[7] = piVar3[7] + 1;
      }
      else {
        uVar8 = (**(code **)(*piVar3 + 0x1c))(bVar2);
      }
      if (uVar8 == 0xffffffff) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
      }
      param_1 = param_1 + -1;
    } while (param_1 != (char *)0x0);
  }
  iVar9 = (**(code **)(**(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this) + 0x14))
                    (param_2,iVar10);
  if (iVar9 != iVar10) {
    *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
         *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
  }
  if (((*(byte *)(*(int *)(*(int *)this + 4) + 0x24 + (int)this) & 2) != 0) && (0 < iVar10)) {
    do {
      bVar2 = *(byte *)(*(int *)(*(int *)this + 4) + 0x2c + (int)this);
      piVar3 = *(int **)((int)this + *(int *)(*(int *)this + 4) + 4);
      if ((byte *)piVar3[7] < (byte *)piVar3[8]) {
        *(byte *)piVar3[7] = bVar2;
        uVar8 = (uint)bVar2;
        piVar3[7] = piVar3[7] + 1;
      }
      else {
        uVar8 = (**(code **)(*piVar3 + 0x1c))(bVar2);
      }
      if (uVar8 == 0xffffffff) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
      }
      iVar10 = iVar10 + -1;
    } while (iVar10 != 0);
  }
  return this;
}

