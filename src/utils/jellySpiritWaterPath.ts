import { calculateWaterY, calculateEllipseIntersection, type WaterPathResult } from './waterLevel'

/**
 * 生成 JellySpirit 的水位路径
 * @param usagePercent 用量百分比 (0-100)
 */
export function generateJellySpiritWaterPath(usagePercent: number): WaterPathResult {
  // JellySpirit viewBox: 0 0 64 64
  const VIEW_BOX = { width: 64, height: 64, bottom: 64 }
  const ELLIPSE = { cx: 32, cy: 36, rx: 24, ry: 26 }

  // 计算水位线 Y 坐标
  const waterY = calculateWaterY(usagePercent, VIEW_BOX.height, VIEW_BOX.bottom)

  // 椭圆顶部和底部 Y 坐标
  const ellipseTop = ELLIPSE.cy - ELLIPSE.ry
  const ellipseBottom = ELLIPSE.cy + ELLIPSE.ry

  // 边界情况：完全空或完全满
  if (waterY >= ellipseBottom) {
    return {
      waterPath: '',
      emptyPath: `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                 A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                 A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`,
      isEmpty: true,
      isFull: false
    }
  }

  if (waterY <= ellipseTop) {
    return {
      waterPath: `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                  A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                  A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 1 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`,
      emptyPath: '',
      isEmpty: false,
      isFull: true
    }
  }

  // 计算水位线与椭圆的交点
  const intersection = calculateEllipseIntersection(waterY, ELLIPSE.cx, ELLIPSE.cy, ELLIPSE.rx, ELLIPSE.ry)

  if (!intersection) {
    return {
      waterPath: '',
      emptyPath: '',
      isEmpty: true,
      isFull: false
    }
  }

  // 生成有水部分路径（下半椭圆 + 水平线）
  const waterPath = `M ${intersection.left} ${waterY}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 0 1 ${intersection.right} ${waterY}
                     L ${intersection.left} ${waterY}`

  // 生成无水部分路径（上半椭圆）
  const emptyPath = `M ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 0 ${ELLIPSE.cx + ELLIPSE.rx} ${ELLIPSE.cy}
                     A ${ELLIPSE.rx} ${ELLIPSE.ry} 0 1 0 ${ELLIPSE.cx - ELLIPSE.rx} ${ELLIPSE.cy}`

  return {
    waterPath,
    emptyPath,
    isEmpty: false,
    isFull: false
  }
}
